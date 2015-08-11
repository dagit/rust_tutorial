extern crate tcod;
use std::cell::RefCell;

use self::tcod::{Console, RootConsole, BackgroundFlag};
use self::tcod::input::KeyState;

use util::Point;

pub struct TcodRenderingComponent {
  pub console: RefCell<RootConsole>
}

impl TcodRenderingComponent {
  pub fn before_render_new_frame(&self) {
    self.console.borrow_mut().clear();
  }

  pub fn render_object(&self, position: &Point, symbol: char) {
    self.console.borrow_mut().put_char(position.x, position.y, symbol, BackgroundFlag::Set);
  }

  pub fn after_render_new_frame(&self) {
    self.console.borrow_mut().flush();
  }

  pub fn wait_for_keypress(&self) -> KeyState {
    self.console.borrow_mut().wait_for_keypress(true)
  }

  pub fn window_closed(&self) -> bool {
    self.console.borrow().window_closed()
  }
}

