use std::cell::RefCell;

use util::Point;
use rendering::RenderingComponent;
use movement::MovementComponent;

pub struct Character<'a> {
  pub position: RefCell<Point>,
  pub display_char: char,
  pub movement_component: Box<MovementComponent + 'a>
}

impl <'a>Character<'a> {
  pub fn new(x: i32, y: i32, dc: char, mc: Box<MovementComponent + 'a>) -> Character<'a> {
    Character { position: RefCell::new(Point { x: x, y: y })
              , display_char: dc, movement_component: mc }
  }

  pub fn update(&self){
    let old_position = self.position.borrow().clone();
    *self.position.borrow_mut() = self.movement_component.update(old_position);
  }

  pub fn render(&self, rendering_component: &RenderingComponent){
    rendering_component.render_object(*self.position.borrow(), self.display_char);
  }
}

