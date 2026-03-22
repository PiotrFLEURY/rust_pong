mod digits;

use std::collections::HashSet;

use ::piston::input::*;
use piston_window::{
    graphics::{Context, clear, rectangle},
    wgpu_graphics::WgpuGraphics,
    *,
};

const WIDTH: f64 = 1024.0;
const HEIGHT: f64 = 768.0;

const MAX_SCORE: u32 = 9;

enum Winner {
    Player,
    AI,
}

struct Game {
    ball: [f64; 4],          // [x, y, dx, dy]
    player_paddle: [f64; 4], // [x, y, width, height]
    ai_paddle: [f64; 4],     // [x, y, width, height]
    scores: [u32; 2],
    ball_acceleration: f64, // Vitesse de la balle
    player_speed: f64,      // Vitesse de déplacement du joueur
    ia_speed: f64,
    level: i8,
    winner: Option<Winner>, // Vitesse de déplacement de l'IA
}

impl Game {
    fn new() -> Self {
        Game {
            ball: [100.0, 100.0, 2.0, 2.0],
            player_paddle: [20.0, 50.0, 10.0, 50.0], // Paddle gauche
            ai_paddle: [WIDTH - 30.0, 50.0, 10.0, 50.0], // Paddle droit
            scores: [0, 0],
            ball_acceleration: 1.0,
            player_speed: 1.0,
            ia_speed: 1.0,
            level: 1,
            winner: None,
        }
    }

    fn update(&mut self, pressed_keys: &HashSet<Key>) {
        if self.scores[0] >= MAX_SCORE || self.scores[1] >= MAX_SCORE {
            // Réinitialiser le jeu
            self.ball_acceleration = 0.0;
            self.player_speed = 0.0;
            self.ia_speed = 0.0;
            self.winner = match self.scores[0] >= MAX_SCORE {
                true => Some(Winner::Player),
                false => Some(Winner::AI),
            };
        }

        // TODO: Afficher un message de victoire et arrêter le jeu
        match self.winner {
            Some(Winner::Player) => {
                // Logique pour afficher le message de victoire du joueur
                // Vous pouvez ajouter du code ici pour dessiner un message à l'écran
                return; // Ne pas continuer la mise à jour du jeu
            }
            Some(Winner::AI) => {
                // Logique pour afficher le message de victoire de l'IA
                // Vous pouvez ajouter du code ici pour dessiner un message à l'écran
                return; // Ne pas continuer la mise à jour du jeu
            }
            None => {} // Continuer le jeu normalement
        }

        // Contrôles du joueur
        if pressed_keys.is_empty() {
            self.player_speed = 1.0; // Réinitialiser la vitesse du joueur lorsque les touches sont relâchées
        } else {
            self.player_speed += 0.02; // Augmenter la vitesse du joueur au fil du temps
        }
        if pressed_keys.contains(&Key::Up) && self.player_paddle[1] > 0.0 {
            self.player_paddle[1] -= self.player_speed; // Déplacer vers le haut
        }
        if pressed_keys.contains(&Key::Down)
            && self.player_paddle[1] + self.player_paddle[3] < HEIGHT
        {
            self.player_paddle[1] += self.player_speed; // Déplacer vers le bas
        }
        if pressed_keys.contains(&Key::Escape) {
            // Restart the game
            self.scores = [0, 0];
            self.ball_acceleration = 1.0;
            self.player_speed = 1.0;
            self.ia_speed = 1.0;
            self.level = 1;
            self.winner = None;
            return;
        }

        // Logique de mouvement de la balle et des raquettes
        self.ball[0] += self.ball[2] * self.ball_acceleration; // Appliquer l'accélération à la vitesse horizontale
        self.ball[1] += self.ball[3] * self.ball_acceleration; // Appliquer l'accélération à la vitesse verticale

        // Exemple de collision avec les bords haut et bas
        if self.ball[1] <= 0.0 || self.ball[1] >= HEIGHT {
            self.ball[3] = -self.ball[3];
        }

        // Collision avec la raquette du joueur
        if self.ball[0] <= self.player_paddle[0] + self.player_paddle[2]
            && self.ball[1] >= self.player_paddle[1]
            && self.ball[1] <= self.player_paddle[1] + self.player_paddle[3]
        {
            self.ball[2] = -self.ball[2];
            self.ball_acceleration += 0.01; // Augmenter l'accélération de la balle au fil du temps
        }

        // Collision avec la raquette de l'IA
        if self.ball[0] + 10.0 >= self.ai_paddle[0]
            && self.ball[1] >= self.ai_paddle[1]
            && self.ball[1] <= self.ai_paddle[1] + self.ai_paddle[3]
        {
            self.ball[2] = -self.ball[2];
            self.ball_acceleration += 0.01; // Augmenter l'accélération de la balle au fil du temps
        }

        // Si la raquette sort des limites on incrémente le score et réinitialise la balle
        if self.ball[0] <= self.player_paddle[0] || self.ball[0] >= self.ai_paddle[0] {
            if self.ball[0] <= self.player_paddle[0] {
                self.scores[1] += 1; // L'IA marque
                // on agrandi la raquette du joueur pour compenser la difficulté accrue
                self.player_paddle[3] += 5.0;
            } else {
                self.scores[0] += 1; // Le joueur marque
            }
            self.ball = [WIDTH / 2.0, HEIGHT / 2.0, 2.0, 2.0]; // Réinitialiser la balle
            // Direction horizontale aléatoire
            self.ball[2] *= if rand::random() { 1.0 } else { -1.0 };
            // Direction verticale aléatoire
            self.ball[3] *= if rand::random() { 1.0 } else { -1.0 };
            self.ball_acceleration = 1.0 + (self.level as f64 / 10.0); // Réinitialiser l'accélération de la balle
            self.level += 1; // Augmenter le niveau de difficulté
        }

        // Logique de mouvement de l'IA (simple suivi de la balle)
        if self.ball[0] > WIDTH / 2.0 {
            if self.ai_paddle[1] + self.ai_paddle[3] / 2.0 < self.ball[1] {
                self.ai_paddle[1] += self.ia_speed; // Déplacer vers le bas
            } else {
                self.ai_paddle[1] -= self.ia_speed; // Déplacer vers le haut
            }
            self.ia_speed += 0.01; // Augmenter la vitesse de la balle au fil du temps
        } else {
            self.ia_speed = 1.0; // Réinitialiser la vitesse de l'IA lorsque la balle est du côté du joueur
        }
    }

    fn draw(&self, c: Context, g: &mut WgpuGraphics) {
        // Dessiner la balle et les raquettes
        rectangle(
            [1.0, 1.0, 1.0, 1.0], // Couleur blanche
            [self.ball[0], self.ball[1], 10.0, 10.0],
            c.transform,
            g,
        );

        rectangle(
            [1.0, 1.0, 1.0, 1.0], // Couleur blanche
            self.player_paddle,
            c.transform,
            g,
        );

        rectangle(
            [1.0, 1.0, 1.0, 1.0], // Couleur blanche
            self.ai_paddle,
            c.transform,
            g,
        );

        // Middle line separator
        for i in (0..HEIGHT as usize).step_by(20) {
            rectangle(
                [1.0, 1.0, 1.0, 1.0], // Couleur blanche
                [WIDTH / 2.0 - 1.0, i as f64, 2.0, 10.0],
                c.transform,
                g,
            );
        }

        // Score
        digits::draw_digit(self.scores[0], 50.0, 20.0, c, g);
        digits::draw_digit(self.scores[1], WIDTH - 70.0, 20.0, c, g);
    }
}

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Pong en Rust", [WIDTH as u32, HEIGHT as u32])
            .exit_on_esc(true)
            .resizable(false)
            .build()
            .unwrap();

    let mut game = Game::new();
    let mut pressed_keys = HashSet::new();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            game.draw(c, g);
        });

        if let Some(Button::Keyboard(key)) = e.press_args() {
            pressed_keys.insert(key);
        }
        if let Some(Button::Keyboard(key)) = e.release_args() {
            pressed_keys.remove(&key);
        }
        if let Some(_) = e.update_args() {
            game.update(&pressed_keys);
        }
    }
}
