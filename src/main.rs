extern crate tcod;
extern crate rand;
extern crate rl;

use tcod::input::Key::Special;
use tcod::input::KeyCode::Escape;

use rl::game::Game;
use rl::character::Character;
use rl::npc::NPC;
use rl::traits::{Updates, Updatable, mk_updatable};
use rl::rendering::RenderingComponent;
use rl::movement::{TcodMovementComponent, RandomMovementComponent};

fn main() {
  let game = Game::new();
  let keypress_fn = &|| game.get_last_keypress();
  let mut c = Character::new(40, 25, '@',
    Box::new(TcodMovementComponent::new(game.window_bounds, keypress_fn)));
  let npcs: Vec<Updatable> = vec![
    mk_updatable(NPC::new(10, 10, 'd',
      Box::new(RandomMovementComponent::new(game.window_bounds)))),
    mk_updatable(NPC::new(40, 25, 'c',
      Box::new(RandomMovementComponent::new(game.window_bounds)))),
    ];
  game.render(&npcs, &c);
  while !(game.rendering_component.window_closed() || *game.exit.borrow()) {
    // wait for user input
    game.wait_for_keypress();
    if let Some(keypress) = game.get_last_keypress() {
      // update game state
      match keypress.key {
        Special(Escape) => game.set_exit(true),
        _               => {}
      };
      game.update(&npcs, &mut c);
    }
    // render
    game.render(&npcs, &c);
  }
}
