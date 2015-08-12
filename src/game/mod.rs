extern crate rand;
extern crate tcod;

use std::cell::RefCell;

use self::tcod::RootConsole;
use self::tcod::input::KeyState;

use util::{Bound, Point};
use rendering::{RenderingComponent, TcodRenderingComponent};
use traits::{Updates, Updatable};
use character::Character;


pub struct Game<'a> {
  pub exit: RefCell<bool>,
  pub window_bounds: Bound,
  pub rendering_component: Box<RenderingComponent + 'a>,
  last_keypress : RefCell<Option<KeyState>>
}

impl<'a> Game<'a> {
  pub fn new() -> Game<'a> {
    let bound = Bound {
        min: Point { x: 0, y: 0 },
        max: Point { x: 79, y: 49 }
    };
    let con = RootConsole::initializer()
      .size(bound.max.x+1, bound.max.y+1)
      .title("libtcod Rust tutorial")
      .init();
    let rc = TcodRenderingComponent { console: RefCell::new(con) };
    Game {
      exit: RefCell::new(false),
      window_bounds: bound,
      rendering_component: Box::new(rc),
      last_keypress: RefCell::new(None)
    }
  }

  pub fn render(&self,
                npcs: &Vec<Updatable<'a>>,
                c: &Character) {
    self.rendering_component.before_render_new_frame();
    for i in npcs.iter() {
      i.borrow().render(&*self.rendering_component);
    }
    c.render(&*self.rendering_component);
    self.rendering_component.after_render_new_frame();
  }

  pub fn update(&self, npcs: &Vec<Updatable<'a>>, c: &Character) {
    c.update();
    for i in npcs.iter() {
      i.borrow().update();
    }
  }

  pub fn wait_for_keypress(&self) {
    *self.last_keypress.borrow_mut() = Some(self.rendering_component.wait_for_keypress());
  }

  pub fn get_last_keypress(&self) -> Option<KeyState> {
    *self.last_keypress.borrow()
  }

  pub fn set_exit(&self, b: bool) {
    *self.exit.borrow_mut() = b;
  }
}

