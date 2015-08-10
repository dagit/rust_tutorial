extern crate tcod;
extern crate rand;
use rand::distributions::{Sample, Range};
use tcod::{Console, RootConsole, BackgroundFlag};
use tcod::input::Key::Special;
use tcod::input::KeyCode::{Escape, Up, Down, Left, Right};

fn render(con: &mut RootConsole, x: i32, y: i32, dog_x: i32, dog_y: i32) {
  con.clear();
  con.put_char(x, y, '@', BackgroundFlag::Set);
  con.put_char(dog_x, dog_y, 'd', BackgroundFlag::Set);
  con.flush();
}

fn main() {
  let mut between = Range::new(0, 3i32);
  let mut rng = rand::thread_rng();
  let con_x = 80i32;
  let con_y = 50i32;
  let mut char_x = 40i32;
  let mut char_y = 25i32;
  let mut dog_x = 10i32;
  let mut dog_y = 10i32;
  let mut con = RootConsole::initializer()
    .size(con_x, con_y)
    .title("libtcod Rust tutorial")
    .init();
  let mut exit = false;
  render(&mut con, char_x, char_y, dog_x, dog_y);
  while !(con.window_closed() || exit) {
    // wait for user input
    let keypress = con.wait_for_keypress(true);

    // update game state
    match keypress.key {
      Special(Escape) => exit = true,
      Special(Up)     => {
        if char_y >= 1 {
          char_y -= 1;
        }
      },
      Special(Down)   => {
        if char_y < (con_y -1) {
          char_y += 1;
        }
      },
      Special(Left) => {
        if char_x >= 1 {
          char_x -= 1;
        }
      },
      Special(Right) => {
        if char_x < (con_x - 1) {
          char_x += 1;
        }
      },
      _               => {}
    }

    let offset_x = between.sample(&mut rng) - 1;
    if(dog_x + offset_x) > 0 && (dog_x + offset_x) < (con_x - 1){
      dog_x += offset_x;
    }

    let offset_y = between.sample(&mut rng) - 1;
    if(dog_y + offset_y) > 0 && (dog_y + offset_y) < (con_y -1){
      dog_y += offset_y;
    }

    // render
    render(&mut con, char_x, char_y, dog_x, dog_y);
  }
}
