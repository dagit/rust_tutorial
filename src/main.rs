extern crate tcod;
use tcod::{Console, RootConsole, BackgroundFlag};
use tcod::input::Key::Special;
use tcod::input::KeyCode::{Escape, Up, Down, Left, Right};

fn render(con: &mut RootConsole, x: i32, y: i32) {
  con.clear();
  con.put_char(x, y, '@', BackgroundFlag::Set);
  con.flush();
}

fn main() {
  let con_x = 80i32;
  let con_y = 50i32;
  let mut char_x = 40i32;
  let mut char_y = 25i32;
  let mut con = RootConsole::initializer()
    .size(con_x, con_y)
    .title("libtcod Rust tutorial")
    .init();
  let mut exit = false;
  render(&mut con, char_x, char_y);
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

    // render
    render(&mut con, char_x, char_y);
  }
}
