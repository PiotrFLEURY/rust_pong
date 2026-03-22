use piston_window::{
    graphics::{Context, rectangle},
    wgpu_graphics::WgpuGraphics,
};

pub fn draw_digit(score: u32, x: f64, y: f64, c: Context, g: &mut WgpuGraphics) {
    let digit_width = 20.0;
    let digit_height = 30.0;
    let thickness = 5.0;

    // Dessiner les segments du chiffre en fonction du score
    match score {
        0 => {
            // Segments pour le chiffre 0
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [x, y, digit_width, thickness],
                c.transform,
                g,
            );
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [x, y + digit_height - thickness, digit_width, thickness],
                c.transform,
                g,
            );
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [x, y, thickness, digit_height],
                c.transform,
                g,
            );
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [x + digit_width - thickness, y, thickness, digit_height],
                c.transform,
                g,
            );
        }
        1 => {
            // Segments pour le chiffre 1
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [x + digit_width - thickness, y, thickness, digit_height],
                c.transform,
                g,
            );
        }
        2 => {
            // Segments pour le chiffre 2
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [x, y, digit_width, thickness],
                c.transform,
                g,
            );
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [
                    x + digit_width - thickness,
                    y + thickness,
                    thickness,
                    thickness,
                ],
                c.transform,
                g,
            );
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [x, y + thickness * 2.0, digit_width, thickness],
                c.transform,
                g,
            );
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [x, y + thickness * 3.0, thickness, thickness],
                c.transform,
                g,
            );
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [x, y + thickness * 4.0, digit_width, thickness],
                c.transform,
                g,
            );
        }
        3 => {
            // Segments pour le chiffre 3
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [x, y, digit_width, thickness],
                c.transform,
                g,
            );
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [
                    x + digit_width - thickness,
                    y + thickness,
                    thickness,
                    thickness,
                ],
                c.transform,
                g,
            );
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [x, y + thickness * 2.0, digit_width, thickness],
                c.transform,
                g,
            );
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [
                    x + digit_width - thickness,
                    y + thickness * 3.0,
                    thickness,
                    thickness,
                ],
                c.transform,
                g,
            );
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [x, y + thickness * 4.0, digit_width, thickness],
                c.transform,
                g,
            );
        }
        4 => {
            // Segments pour le chiffre 4
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [x, y, thickness, digit_height / 2.0],
                c.transform,
                g,
            );
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [x, y + digit_height / 2.0, digit_width, thickness],
                c.transform,
                g,
            );
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [x + digit_width - thickness, y, thickness, digit_height],
                c.transform,
                g,
            );
        }
        5 => {
            // Segments pour le chiffre 5
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [x, y, digit_width, thickness],
                c.transform,
                g,
            );
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [x, y + thickness, thickness, thickness],
                c.transform,
                g,
            );
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [x, y + thickness * 2.0, digit_width, thickness],
                c.transform,
                g,
            );
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [
                    x + digit_width - thickness,
                    y + thickness * 3.0,
                    thickness,
                    thickness,
                ],
                c.transform,
                g,
            );
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [x, y + thickness * 4.0, digit_width, thickness],
                c.transform,
                g,
            );
        }
        6 => {
            // Segments pour le chiffre 6
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [x, y, thickness, digit_height],
                c.transform,
                g,
            );
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [
                    x + thickness,
                    y + thickness * 2.0,
                    digit_width - thickness,
                    thickness,
                ],
                c.transform,
                g,
            );
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [
                    x + digit_width - thickness,
                    y + thickness * 3.0,
                    thickness,
                    thickness * 2.0,
                ],
                c.transform,
                g,
            );
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [
                    x + thickness,
                    y + digit_height - thickness,
                    digit_width - thickness,
                    thickness,
                ],
                c.transform,
                g,
            );
        }
        7 => {
            // Segments pour le chiffre 7
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [x, y, digit_width, thickness],
                c.transform,
                g,
            );
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [
                    x + digit_width - thickness,
                    y + thickness,
                    thickness,
                    digit_height - thickness,
                ],
                c.transform,
                g,
            );
        }
        8 => {
            // Segments pour le chiffre 8
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [x, y, digit_width, thickness],
                c.transform,
                g,
            );
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [x, y + thickness * 2.0, digit_width, thickness],
                c.transform,
                g,
            );
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [x, y + thickness * 4.0, digit_width, thickness],
                c.transform,
                g,
            );
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [x, y + thickness, thickness, digit_height - thickness * 2.0],
                c.transform,
                g,
            );
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [
                    x + digit_width - thickness,
                    y + thickness,
                    thickness,
                    digit_height - thickness * 2.0,
                ],
                c.transform,
                g,
            );
        }
        9 => {
            // Segments pour le chiffre 9
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [x, y, digit_width, thickness],
                c.transform,
                g,
            );
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [x, y + thickness, thickness, thickness],
                c.transform,
                g,
            );
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [x, y + thickness * 2.0, digit_width, thickness],
                c.transform,
                g,
            );
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [
                    x + digit_width - thickness,
                    y + thickness,
                    thickness,
                    digit_height - thickness,
                ],
                c.transform,
                g,
            );
        }
        // Ajouter les segments pour les autres chiffres (2-9) ici
        _ => {}
    }
}
