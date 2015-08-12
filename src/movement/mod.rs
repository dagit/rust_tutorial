extern crate rand;
extern crate tcod;

use self::rand::thread_rng;
use self::rand::distributions::{Range, Sample};

use self::tcod::input::KeyState;
use self::tcod::input::Key::Special;
use self::tcod::input::KeyCode::{Up, Left, Down, Right};

use util::{Point, Bound, Contains};

pub trait MovementComponent {
  fn update(&self, Point) -> Point;
}

pub struct RandomMovementComponent {
  pub window_bounds: Bound
}

impl RandomMovementComponent {
  pub fn new(bounds: Bound) -> RandomMovementComponent {
    RandomMovementComponent { window_bounds: bounds }
  }
}

impl MovementComponent for RandomMovementComponent {
  fn update(&self, point: Point) -> Point {
    let mut offset = Point { x: point.x, y: point.y };
    let mut between = Range::new(0, 3i32);
    let offset_x = between.sample(&mut thread_rng()) - 1;

    offset = match self.window_bounds.contains(offset.offset_x(offset_x)) {
      Contains::DoesContain    => offset.offset_x(offset_x),
      Contains::DoesNotContain => point
    };

    let offset_y = between.sample(&mut thread_rng()) - 1;
    match self.window_bounds.contains(offset.offset_y(offset_y)) {
      Contains::DoesContain    => offset.offset_y(offset_y),
      Contains::DoesNotContain => point
    }
  }
}

pub struct TcodMovementComponent<'a> {
  pub window_bounds: Bound,
  pub get_last_keypress: &'a Fn() -> Option<KeyState>
}

impl<'a> TcodMovementComponent<'a> {
  pub fn new(bounds: Bound, glk: &'a Fn() -> Option<KeyState>) -> TcodMovementComponent<'a> {
    TcodMovementComponent { window_bounds: bounds, get_last_keypress: glk }
  }
}

impl<'a> MovementComponent for TcodMovementComponent<'a> {
  fn update(&self, point: Point) -> Point {
    let mut offset = point.clone();

    offset = match (self.get_last_keypress)() {
      Some(p) => match p.key {
        Special(Up) => {
          offset.offset_y(-1)
        },
        Special(Down) => {
          offset.offset_y(1)
        },
        Special(Left) => {
          offset.offset_x(-1)
        },
        Special(Right) => {
          offset.offset_x(1)
        },
        _ => offset
      },
      _ => offset
    };

    match self.window_bounds.contains(offset) {
      Contains::DoesContain    => offset,
      Contains::DoesNotContain => point
    }
  }
}



