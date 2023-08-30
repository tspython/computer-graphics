extern crate minifb;

use minifb::{Key, Window, WindowOptions};

fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 600;

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Rainbow Triangle - Press Esc to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for pixel in &mut buffer {
            *pixel = 0;
        }

        let colors = [
            0xFF0000, // Red
            0xFF7F00, // Orange
            0xFFFF00, // Yellow
            0x00FF00, // Green
            0x0000FF, // Blue
            0x4B0082, // Indigo
            0x9400D3, // Violet
        ];

        let vertices = [
            (400.0, 100.0),
            (200.0, 500.0),
            (600.0, 500.0),
        ];

        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let p = (x as f64, y as f64);
                let a = vertices[0];
                let b = vertices[1];
                let c = vertices[2];
                let det = (b.1 - c.1) * (a.0 - c.0) + (c.0 - b.0) * (a.1 - c.1);
                let alpha = ((b.1 - c.1) * (p.0 - c.0) + (c.0 - b.0) * (p.1 - c.1)) / det;
                let beta = ((c.1 - a.1) * (p.0 - c.0) + (a.0 - c.0) * (p.1 - c.1)) / det;
                let gamma = 1.0 - alpha - beta;
                if alpha >= 0.0 && beta >= 0.0 && gamma >= 0.0 {
                    let color = interpolate_color(&colors, alpha, beta, gamma);
                    buffer[y * WIDTH + x] = color;
                }
            }
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn interpolate_color(colors: &[u32; 7], alpha: f64, beta: f64, gamma: f64) -> u32 {
    let mut color_index = (alpha * (colors.len() - 1) as f64).floor() as usize;
    if color_index >= colors.len() - 1 {
        color_index = colors.len() - 2;
    }

    let remainder = alpha * (colors.len() - 1) as f64 - color_index as f64;

    let r = ((colors[color_index] >> 16) & 0xFF) as f64 * (1.0 - remainder)
        + ((colors[color_index + 1] >> 16) & 0xFF) as f64 * remainder;

    let g = ((colors[color_index] >> 8) & 0xFF) as f64 * (1.0 - remainder)
        + ((colors[color_index + 1] >> 8) & 0xFF) as f64 * remainder;

    let b = (colors[color_index] & 0xFF) as f64 * (1.0 - remainder)
        + (colors[color_index + 1] & 0xFF) as f64 * remainder;

    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

