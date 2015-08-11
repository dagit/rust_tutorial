extern crate tcod;
use self::tcod::{Console, RootConsole, BackgroundFlag};
use self::tcod::input::KeyCode::{Up, Down, Left, Right};
use self::tcod::input::Key::Special;
use self::tcod::input::KeyState;

use traits::Updates;
use util::{Point, Contains};
use game::Game;

pub struct Character {
  pub position: Point,
  pub display_char: char
}

impl Character {
  pub fn new(x: i32, y: i32, dc: char) -> Character {
    Character { position: Point { x: x, y: y }, display_char: dc }
  }
}

impl Updates for Character {
  fn update(&mut self, keypress: KeyState, game: &Game){
    let mut offset = Point { x: 0, y: 0 };
    match keypress.key {
      Special(Up) => {
        offset.y = -1;
      },
      Special(Down) => {
        offset.y = 1;
      },
      Special(Left) => {
        offset.x = -1;
      },
      Special(Right) => {
        offset.x = 1;
      },
      _ => {}
    }

    match game.window_bounds.contains(self.position.offset(&offset)){
      Contains::DoesContain    => self.position = self.position.offset(&offset),
      Contains::DoesNotContain => {}
    }
  }

  fn render(&self, console: &mut RootConsole){
    console.put_char(self.position.x, self.position.y,
                     self.display_char, BackgroundFlag::Set);
  }
}

