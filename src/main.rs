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

fn render<'a>(con: &mut RootConsole, npcs: &Vec<Updatable<'a>>, c: &Character) {
  con.clear();
  for i in npcs.iter() {
    i.borrow().render(con);
  }
  c.render(con);
  con.flush();
}

fn update<'a>(npcs: &Vec<Updatable<'a>>, c: &mut Character, keypress: KeyState, game: &Game) {
  c.update(keypress, &game);
  for i in npcs.iter() {
    i.borrow_mut().update(&game);
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
  let mut c = Character::new(40, 25, '@');
  let npcs: Vec<Updatable> = vec![
    mk_updatable(NPC::new(10, 10, 'd')),
    mk_updatable(NPC::new(40, 25, 'c')),
    ];
  let mut con = RootConsole::initializer()
    .size(game.window_bounds.max.x+1, game.window_bounds.max.y+1)
    .title("libtcod Rust tutorial")
    .init();
  render(&mut con, &npcs, &c);
  while !(con.window_closed() || game.exit) {
    // wait for user input
    let keypress = con.wait_for_keypress(true);

    // update game state
    match keypress.key {
      Special(Escape) => game.exit = true,
      _               => {}
    }
    update(&npcs, &mut c, keypress, &game);

    // render
    render(&mut con, &npcs, &c);
  }
}
