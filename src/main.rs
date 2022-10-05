use sfml::graphics::{Texture, RenderWindow, Color, RenderTarget, Sprite, Transformable, Font, Text};
use sfml::system::Vector2f;
use sfml::window::{VideoMode, Style, ContextSettings, Event, Key};

static WINDOW_RESOLUTION_X: f32 = 640.0;
static WINDOW_RESOLUTION_Y: f32 = 420.0;

struct ScoreBoard {
  score_right: u32,
  score_left: u32,
}

fn reset_ball_position(ball: &mut Sprite) {
  ball.set_position(Vector2f::new(
    WINDOW_RESOLUTION_X / 2.0,
    WINDOW_RESOLUTION_Y / 2.0,
  ));
}

fn collision_ball(ball: &Sprite, paddle: &Sprite) -> bool {
  paddle
    .global_bounds()
    .contains(Vector2f::new(ball.position().x, ball.position().y))
}

fn main() {
  let mut speed_x: f32 = 2.0;
  let mut speed_y: f32 = 2.0;
  let mut score_board: ScoreBoard = ScoreBoard {
    score_right: 0,
    score_left: 0,
  };

  let font = Font::from_file("fonts/Pixelar.ttf").expect("Error load font");
  let mut score_right_text = Text::new(&score_board.score_right.to_string(), &font, 100);
  let mut score_left_text = Text::new(&score_board.score_left.to_string(), &font, 100);

  score_right_text.set_position(Vector2f::new(
    (WINDOW_RESOLUTION_X / 2.0) + (WINDOW_RESOLUTION_X / 2.0) / 2.0,
    25.0,
  ));
  score_left_text.set_position(Vector2f::new((WINDOW_RESOLUTION_X / 2.0) / 2.0, 25.0));

  // Load texture the ball
  let ball_texture = Texture::from_file("assets/ball.png").expect("Error load texture ball");

  // Load texture the paddle
  let paddle_texture = Texture::from_file("assets/paddle.png").expect("Error load texture paddle");

  // Set Texture in sprite
  let mut ball_sprite: Sprite = Sprite::with_texture(&ball_texture);
  let mut paddle_sprite_right = Sprite::with_texture(&paddle_texture);
  let mut paddle_sprite_left = Sprite::with_texture(&paddle_texture);

  // Set position origin
  ball_sprite.set_origin(Vector2f::new(
    (ball_texture.size().x / 2) as f32,
    (ball_texture.size().y / 2) as f32,
  ));

  paddle_sprite_right.set_position(Vector2f::new(WINDOW_RESOLUTION_X - 25.0, 200.0));
  paddle_sprite_right.set_rotation(-90.0);

  paddle_sprite_left.set_position(Vector2f::new(25.0, 200.0));
  paddle_sprite_left.set_rotation(90.0);

  // Set scale
  ball_sprite.scale(Vector2f::new(2.25, 2.25));

  paddle_sprite_right.set_scale(Vector2f::new(2.0, 2.0));
  paddle_sprite_left.set_scale(Vector2f::new(2.0, 2.0));

  // Set position in the window
  reset_ball_position(&mut ball_sprite);

  let mut window = RenderWindow::new(
    VideoMode::desktop_mode(),
    "Pong",
    Style::default(),
    &ContextSettings::default(),
  );

  window.set_framerate_limit(60);

  while window.is_open() {
    while let Some(event) = window.poll_event() {
      match event {
        Event::Closed => window.close(),
        _ => (),
      }
    }

    // Start move the ball
    ball_sprite.move_(Vector2f::new(speed_x, speed_y));

    if collision_ball(&ball_sprite, &paddle_sprite_right)
      || collision_ball(&ball_sprite, &paddle_sprite_left)
    {
      speed_x *= -1.0;
    }
    if ball_sprite.position().x > WINDOW_RESOLUTION_X {
      score_board.score_left += 1;
      speed_x *= -1.0;
      reset_ball_position(&mut ball_sprite);
      score_left_text.set_string(&score_board.score_left.to_string());
    }

    if ball_sprite.position().x < 0.0 {
      score_board.score_right += 1;
      speed_x *= -1.0;
      reset_ball_position(&mut ball_sprite);
      score_right_text.set_string(&score_board.score_right.to_string());
    }

    if ball_sprite.position().y > WINDOW_RESOLUTION_Y || ball_sprite.position().y < 0.0 {
      speed_y *= -1.0;
    }

    if Key::is_pressed(Key::I) && paddle_sprite_right.position().y > 0.0 {
      paddle_sprite_right.move_(Vector2f::new(0.0, -5.0));
    }

    if Key::is_pressed(Key::K) && paddle_sprite_right.position().y < WINDOW_RESOLUTION_Y {
      paddle_sprite_right.move_(Vector2f::new(0.0, 5.0));
    }

    if Key::is_pressed(Key::W) && paddle_sprite_left.position().y > 0.0 {
      paddle_sprite_left.move_(Vector2f::new(0.0, -5.0));
    }

    if Key::is_pressed(Key::S) && paddle_sprite_left.position().y < WINDOW_RESOLUTION_Y {
      paddle_sprite_left.move_(Vector2f::new(0.0, 5.0));
    }

    window.clear(Color::BLACK);
    window.draw(&ball_sprite);
    window.draw(&paddle_sprite_right);
    window.draw(&paddle_sprite_left);
    window.draw(&score_right_text);
    window.draw(&score_left_text);
    window.display();
  }
}
