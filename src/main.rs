extern crate tcod;
extern crate rand;
use rand::distributions::{Sample, Range};
use tcod::{Console, RootConsole, BackgroundFlag};
use tcod::input::Key::Special;
use tcod::input::KeyCode::{Escape, Up, Down, Left, Right};
use tcod::input::KeyState;
use std::cell::RefCell;

struct Point {
  x: i32,
  y: i32
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

struct Bound {
  min: Point,
  max: Point
}

struct Character {
  position: Point,
  display_char: char
}

impl Character {
  fn new(x: i32, y: i32, dc: char) -> Character {
    Character { position: Point { x: x, y: y }, display_char: dc }
  }
}

struct NPC {
  position: Point,
  display_char: char
}

impl NPC {
  fn new(x: i32, y: i32, dc: char) -> NPC {
    NPC { position: Point { x: x, y: y }, display_char: dc }
  }
}

trait Updates {
  fn update(&mut self, KeyState, &Game);
  fn render(&self, &mut RootConsole);
}

impl Updates for Character {
  fn update(&mut self, keypress: KeyState, game: &Game){
    let mut offset = Point { x: 0, y: 0 };
    match keypress.key {
      Special(Up) => {
        offset.y = -1;
      },
      Special(Down) => {
        offset.y = 1;
      },
      Special(Left) => {
        offset.x = -1;
      },
      Special(Right) => {
        offset.y = 1;
      },
      _ => {}
    }

    match game.window_bounds.contains(self.position.offset(&offset)){
      Contains::DoesContain    => self.position = self.position.offset(&offset),
      Contains::DoesNotContain => {}
    }
  }

  fn render(&self, console: &mut RootConsole){
    console.put_char(self.position.x, self.position.y,
                     self.display_char, BackgroundFlag::Set);
  }
}

impl Updates for NPC {
  fn update(&mut self, keypress: KeyState, game: &Game){
    let mut between = Range::new(0, 3i32);
    let mut rng = rand::thread_rng();
    let offset_x = between.sample(&mut rng) - 1;
    match game.window_bounds.contains(self.position.offset_x(&offset_x)) {
      Contains::DoesContain    => self.position = self.position.offset_x(&offset_x),
      Contains::DoesNotContain => {}
    }

    let offset_y = between.sample(&mut rng) - 1;
    match game.window_bounds.contains(self.position.offset_y(&offset_y)) {
      Contains::DoesContain    => self.position = self.position.offset_y(&offset_y),
      Contains::DoesNotContain => {}
    }
  }

  fn render(&self, console: &mut RootConsole) {
    console.put_char(self.position.x, self.position.y,
             self.display_char, BackgroundFlag::Set);
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

struct Game {
  exit: bool,
  window_bounds: Bound
}

type Updatable<'a> = Box<RefCell<Updates + 'a>>;

fn mk_updatable<'a, Upd: Updates + 'a>(t: Upd) -> Updatable<'a> {
  Box::new(RefCell::new(t)) as Box<RefCell<Updates>>
}

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
      max: Point { x: 79, y: 49 } } };
  let objs: Vec<Updatable> = vec![
    mk_updatable(Character::new(40, 25, '@')),
    mk_updatable(Character::new(10, 10, 'd')),
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
    let mut offset = Point { x: 0, y: 0 };
    match keypress.key {
      Special(Escape) => game.exit = true,
      _               => {}
    }
    update(&objs, keypress, &game);

    // render
    render(&mut con, &objs);
  }
}
