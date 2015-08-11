extern crate tcod;
extern crate rand;
extern crate rl;

use tcod::{Console, RootConsole};
use tcod::input::Key::Special;
use tcod::input::KeyCode::Escape;
use tcod::input::KeyState;
use std::cell::RefCell;

use rl::util::{Point, Bound};
use rl::game::Game;
use rl::character::Character;
use rl::npc::NPC;
use rl::traits::{Updates, Updatable, mk_updatable};

fn render<'a>(con: &mut RootConsole, objs: &Vec<Updatable<'a>>) {
  con.clear();
  for i in objs.iter() {
    i.borrow().render(con);
  }
  con.flush();
}

fn update<'a>(objs: &Vec<Updatable<'a>>, keypress: KeyState, game: &Game) {
  for i in objs.iter() {
    i.borrow_mut().update(keypress, &game);
  }
}

fn main() {
  let mut game = Game {
    exit: false,
    window_bounds: Bound {
      min: Point { x: 0, y: 0 },
      max: Point { x: 79, y: 49 } },
    rng: RefCell::new(rand::thread_rng())
    };
  let objs: Vec<Updatable> = vec![
    mk_updatable(Character::new(40, 25, '@')),
    mk_updatable(NPC::new(10, 10, 'd')),
    ];
  let mut con = RootConsole::initializer()
    .size(game.window_bounds.max.x+1, game.window_bounds.max.y+1)
    .title("libtcod Rust tutorial")
    .init();
  render(&mut con, &objs);
  while !(con.window_closed() || game.exit) {
    // wait for user input
    let keypress = con.wait_for_keypress(true);

    // update game state
    match keypress.key {
      Special(Escape) => game.exit = true,
      _               => {}
    }
    update(&objs, keypress, &game);

    // render
    render(&mut con, &objs);
  }
}
