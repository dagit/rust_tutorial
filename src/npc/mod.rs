extern crate tcod;

use std::cell::RefCell;

use traits::Updates;
use util::{Point};
use rendering::RenderingComponent;
use movement::MovementComponent;

pub struct NPC {
  pub position: RefCell<Point>,
  pub display_char: char,
  pub movement_component: Box<MovementComponent>
}

impl NPC {
  pub fn new(x: i32, y: i32, dc: char, mc: Box<MovementComponent>) -> NPC {
    NPC { position: RefCell::new(Point { x: x, y: y })
        , display_char: dc, movement_component: mc }
  }
}

impl Updates for NPC {
  fn update(&self){
    let old_position = self.position.borrow().clone();
    *self.position.borrow_mut() = self.movement_component.update(old_position);
  }

  fn render(&self, rendering_component: &RenderingComponent) {
    rendering_component.render_object(*self.position.borrow(), self.display_char);
  }
}

