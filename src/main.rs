extern crate tcod;
extern crate rand;
extern crate rl;

use tcod::input::Key::Special;
use tcod::input::KeyCode::Escape;

use rl::game::Game;
use rl::character::Character;
use rl::npc::NPC;
use rl::traits::{Updates, Updatable, mk_updatable};
use rl::rendering::{RenderingComponent};

fn main() {
  let mut c = Character::new(40, 25, '@');
  let npcs: Vec<Updatable> = vec![
    mk_updatable(NPC::new(10, 10, 'd')),
    mk_updatable(NPC::new(40, 25, 'c')),
    ];
  let mut game = Game::new();
  game.render(&npcs, &c);
  while !(game.rendering_component.window_closed() || game.exit) {
    // wait for user input
    let keypress = game.rendering_component.wait_for_keypress();

    // update game state
    match keypress.key {
      Special(Escape) => game.exit = true,
      _               => {}
    }
    game.update(&npcs, &mut c, keypress);

    // render
    game.render(&npcs, &c);
  }
}
