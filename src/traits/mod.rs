extern crate tcod;
use self::tcod::RootConsole;
use game::Game;
use std::cell::RefCell;

pub trait Updates {
  fn update(&mut self, &Game);
  fn render(&self, &mut RootConsole);
}

pub type Updatable<'a> = Box<RefCell<Updates + 'a>>;

pub fn mk_updatable<'a, Upd: Updates + 'a>(t: Upd) -> Updatable<'a> {
  Box::new(RefCell::new(t)) as Box<RefCell<Updates>>
}
