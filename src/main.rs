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

fn render(con: &mut RootConsole, c_point: &Point, d_point: &Point) {
  con.clear();
  con.put_char(c_point.x, c_point.y, '@', BackgroundFlag::Set);
  con.put_char(d_point.x, d_point.y, 'd', BackgroundFlag::Set);
  con.flush();
}

fn main() {
  let mut between = Range::new(0, 3i32);
  let mut rng = rand::thread_rng();
  let con_x = 80i32;
  let con_y = 50i32;
  let mut char_point = Point { x: 40, y: 25 };
  let mut dog_point  = Point { x: 10, y: 10 };
  let mut con = RootConsole::initializer()
    .size(con_x, con_y)
    .title("libtcod Rust tutorial")
    .init();
  let mut exit = false;
  render(&mut con, &char_point, &dog_point);
  while !(con.window_closed() || exit) {
    // wait for user input
    let keypress = con.wait_for_keypress(true);

    // update game state
    match keypress.key {
      Special(Escape) => exit = true,
      Special(Up)     => {
        if char_point.y >= 1 {
          char_point.y -= 1;
        }
      },
      Special(Down)   => {
        if char_point.y < (con_y -1) {
          char_point.y += 1;
        }
      },
      Special(Left) => {
        if char_point.x >= 1 {
          char_point.x -= 1;
        }
      },
      Special(Right) => {
        if char_point.x < (con_x - 1) {
          char_point.x += 1;
        }
      },
      _               => {}
    }

    let offset_x = between.sample(&mut rng) - 1;
    if(dog_point.x + offset_x) > 0 && (dog_point.x + offset_x) < (con_x - 1){
      dog_point.x += offset_x;
    }

    let offset_y = between.sample(&mut rng) - 1;
    if(dog_point.y + offset_y) > 0 && (dog_point.y + offset_y) < (con_y -1){
      dog_point.y += offset_y;
    }

    // render
    render(&mut con, &char_point, &dog_point);
  }
}
