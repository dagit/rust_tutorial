extern crate tcod;
use std::cell::RefCell;

use rendering::RenderingComponent;

pub trait Updates {
  fn update(&self);
  fn render(&self, &RenderingComponent);
}

pub type Updatable<'a> = Box<RefCell<Updates + 'a>>;

pub fn mk_updatable<'a, Upd: Updates + 'a>(t: Upd) -> Updatable<'a> {
  Box::new(RefCell::new(t)) as Box<RefCell<Updates>>
}
