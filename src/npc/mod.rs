extern crate tcod;
extern crate rand;
use self::tcod::{Console, RootConsole, BackgroundFlag};
use self::tcod::input::KeyState;
use self::rand::distributions::{Sample, Range};

use traits::Updates;
use util::{Point, Contains};
use game::Game;

pub struct NPC {
  pub position: Point,
  pub display_char: char
}

impl NPC {
  pub fn new(x: i32, y: i32, dc: char) -> NPC {
    NPC { position: Point { x: x, y: y }, display_char: dc }
  }
}

impl Updates for NPC {
  fn update(&mut self, _: KeyState, game: &Game){
    let mut between = Range::new(0, 3i32);
    let offset_x = between.sample(&mut *game.rng.borrow_mut()) - 1;
    match game.window_bounds.contains(self.position.offset_x(&offset_x)) {
      Contains::DoesContain    => self.position = self.position.offset_x(&offset_x),
      Contains::DoesNotContain => {}
    }

    let offset_y = between.sample(&mut *game.rng.borrow_mut()) - 1;
    match game.window_bounds.contains(self.position.offset_y(&offset_y)) {
      Contains::DoesContain    => self.position = self.position.offset_y(&offset_y),
      Contains::DoesNotContain => {}
    }
  }

  fn render(&self, console: &mut RootConsole) {
    console.put_char(self.position.x, self.position.y,
             self.display_char, BackgroundFlag::Set);
  }
}
