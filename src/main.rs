extern crate tcod;
extern crate rand;
use rand::distributions::{Sample, Range};
use tcod::{Console, RootConsole, BackgroundFlag};
use tcod::input::Key::Special;
use tcod::input::KeyCode::{Escape, Up, Down, Left, Right};

struct Point {
  x: i32,
  y: i32
}

struct Bound {
  min: Point,
  max: Point
}

impl Point {
  fn offset_x(&self, offset: &i32) -> Point {
    Point { x: self.x + offset, y: self.y }
  }

  fn offset_y(&self, offset: &i32) -> Point {
    Point { x: self.x, y: self.y + offset }
  }

  fn offset(&self, offset: &Point) -> Point {
    Point { x: self.x + offset.x, y: self.y + offset.y }
  }
}

enum Contains {
  DoesContain,
  DoesNotContain
}

impl Bound{
  fn contains(&self, point: Point) -> Contains {
    if
      point.x >= self.min.x &&
      point.x <= self.max.x &&
      point.y >= self.min.y &&
      point.y <= self.max.y
    {
      Contains::DoesContain
    } else {
      Contains::DoesNotContain
    }
  }
}

fn render(con: &mut RootConsole, c_point: &Point, d_point: &Point) {
  con.clear();
  con.put_char(c_point.x, c_point.y, '@', BackgroundFlag::Set);
  con.put_char(d_point.x, d_point.y, 'd', BackgroundFlag::Set);
  con.flush();
}

fn main() {
  let mut between = Range::new(0, 3i32);
  let mut rng = rand::thread_rng();
  let window_bounds = Bound { min: Point { x:  0, y:  0 }
                            , max: Point { x: 79, y: 49 }
                            };
  let mut char_point = Point { x: 40, y: 25 };
  let mut dog_point  = Point { x: 10, y: 10 };
  let mut con = RootConsole::initializer()
    .size(window_bounds.max.x+1, window_bounds.max.y+1)
    .title("libtcod Rust tutorial")
    .init();
  let mut exit = false;
  render(&mut con, &char_point, &dog_point);
  while !(con.window_closed() || exit) {
    // wait for user input
    let keypress = con.wait_for_keypress(true);

    // update game state
    let mut offset = Point { x: 0, y: 0 };
    match keypress.key {
      Special(Escape) => exit = true,
      Special(Up)     => {
        offset.y = -1;
      },
      Special(Down)   => {
        offset.y = 1;
      },
      Special(Left) => {
        offset.x = -1;
      },
      Special(Right) => {
        offset.x = 1;
      },
      _               => {}
    }

    match window_bounds.contains(char_point.offset(&offset)) {
      Contains::DoesContain    => char_point = char_point.offset(&offset),
      Contains::DoesNotContain => {}
    }

    let offset_x = between.sample(&mut rng) - 1;
    match window_bounds.contains(dog_point.offset_x(&offset_x)) {
      Contains::DoesContain    => dog_point = dog_point.offset_x(&offset_x),
      Contains::DoesNotContain => {}
    }

    let offset_y = between.sample(&mut rng) - 1;
    match window_bounds.contains(dog_point.offset_y(&offset_y)) {
      Contains::DoesContain    => dog_point = dog_point.offset_y(&offset_y),
      Contains::DoesNotContain => {}
    }

    // render
    render(&mut con, &char_point, &dog_point);
  }
}
