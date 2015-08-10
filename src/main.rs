extern crate tcod;
use tcod::{Console, RootConsole, BackgroundFlag};
use tcod::input::Key::Special;
use tcod::input::KeyCode::{Escape};

fn render(con: &mut RootConsole) {
  con.clear();
  con.put_char(40, 25, '@', BackgroundFlag::Set);
  con.flush();
}

fn main() {
  let mut con = RootConsole::initializer()
    .size(80, 50)
    .title("libtcod Rust tutorial")
    .init();
  let mut exit = false;
  render(&mut con);
  while !(con.window_closed() || exit) {
    // wait for user input
    let keypress = con.wait_for_keypress(true);

    // update game state
    match keypress.key {
      Special(Escape) => exit = true,
      _               => {}
    }

    // render
    render(&mut con);
  }
}
