extern crate rand;

use std::cell::RefCell;
use self::rand::ThreadRng;

use util::Bound;

pub struct Game {
  pub exit: bool,
  pub window_bounds: Bound,
  pub rng: RefCell<ThreadRng>
}

