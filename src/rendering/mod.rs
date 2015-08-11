extern crate tcod;
use std::cell::RefCell;

use self::tcod::{Console, RootConsole, BackgroundFlag};
use self::tcod::input::KeyState;

use util::Point;

pub trait RenderingComponent {
  fn before_render_new_frame(&self);
  fn render_object(&self, &Point, char);
  fn after_render_new_frame(&self);
  fn wait_for_keypress(&self) -> KeyState;
  fn window_closed(&self) -> bool;
}

pub struct TcodRenderingComponent {
  pub console: RefCell<RootConsole>
}

impl RenderingComponent for TcodRenderingComponent {
  fn before_render_new_frame(&self) {
    self.console.borrow_mut().clear();
  }

  fn render_object(&self, position: &Point, symbol: char) {
    self.console.borrow_mut().put_char(position.x, position.y, symbol, BackgroundFlag::Set);
  }

  fn after_render_new_frame(&self) {
    self.console.borrow_mut().flush();
  }

  fn wait_for_keypress(&self) -> KeyState {
    self.console.borrow_mut().wait_for_keypress(true)
  }

  fn window_closed(&self) -> bool {
    self.console.borrow().window_closed()
  }
}

