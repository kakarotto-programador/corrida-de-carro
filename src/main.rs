use std::error::Error;

use ncurses::*;
use rand::prelude::*;

struct Player<'b> {
    col: usize,
    lin: usize,
    is_alive: bool,
    repr: &'b str,
}

impl<'b> Player<'b> {
    fn move_position(&mut self, pos: i32) {
        match pos {
            KEY_LEFT => self.col = if self.col <= 1 { 1 } else { self.col - 1 },
            KEY_RIGHT => self.col = if self.col >= 3 { 3 } else { self.col + 1 },
            _ => {}
        }
    }
}

impl<'b> Default for Player<'b> {
    fn default() -> Self {
        Player {
            col: 2,
            lin: 9,
            is_alive: true,
            repr: "C",
        }
    }
}

struct Enemy<'a> {
    col: usize,
    lin: usize,
    repr: &'a str,
}

impl<'a> Enemy<'a> {
    fn new(mut rng: ThreadRng) -> Self {
        Enemy {
            col: rng.gen_range(1, 4),
            lin: 0,
            repr: "Z",
        }
    }
}

struct World<'c> {
    road: Vec<Vec<&'c str>>,
}

impl<'c> World<'c> {
    fn new() -> Self {
        World {
            road: vec![
                vec!["|", " ", " ", " ", "|"],
                vec!["|", " ", " ", " ", "|"],
                vec!["|", " ", " ", " ", "|"],
                vec!["|", " ", " ", " ", "|"],
                vec!["|", " ", " ", " ", "|"],
                vec!["|", " ", " ", " ", "|"],
                vec!["|", " ", " ", " ", "|"],
                vec!["|", " ", " ", " ", "|"],
                vec!["|", " ", " ", " ", "|"],
                vec!["|", " ", " ", " ", "|"],
                vec!["|", " ", " ", " ", "|"],
            ],
        }
    }

    fn draw_field(
        &self,
        field: &Vec<&str>,
        line: usize,
        player: &mut Player,
        enemy: &Enemy,
    ) -> Result<String, Box<dyn Error>> {
        let row = field
            .iter()
            .enumerate()
            .map(|(f, n)| {
                if (line == player.lin) && (f == player.col) && (player.col >= 1 && player.col <= 3)
                {
                    player.repr
                } else if line == enemy.lin && f == enemy.col {
                    enemy.repr
                } else if enemy.lin == player.lin && enemy.col == player.col {
                    player.is_alive = false;
                    n
                } else {
                    n
                }
            })
            .collect::<Vec<&str>>()
            .join("");
        Ok(row)
    }

    fn draw_road(&self, player: &mut Player, enemy: &Enemy) -> Result<(), Box<dyn Error>> {
        for (i, field) in self.road.iter().enumerate() {
            mvaddstr(i as i32, 0, &self.draw_field(field, i, player, enemy)?);
        }
        Ok(())
    }
}

struct GameState<'a, 'b, 'c> {
    is_alive: bool,
    score: usize,
    enemy_line: usize,
    rng: ThreadRng,
    player: Player<'b>,
    enemy: Enemy<'a>,
    world: World<'c>,
}

impl<'a, 'b, 'c> GameState<'a, 'b, 'c> {
    fn new(rng: ThreadRng) -> Self {
        let world = World::new();
        GameState {
            is_alive: true,
            enemy_line: 0,
            score: 0,
            rng: rng,
            player: Player::default(),
            enemy: Enemy::new(rng),
            world,
        }
    }
    fn update(&mut self) -> Result<(), Box<dyn Error>> {
        self.enemy.lin = self.enemy_line;
        if self.enemy_line == 0 {
            self.enemy.col = self.rng.gen_range(1, 4);
        }
        self.is_alive = self.player.is_alive;
        if self.enemy_line == self.world.road.len() - 1 && self.is_alive {
            self.score += 1
        }
        mvprintw(15, 0, &format!("score: {}", self.score));
        Ok(())
    }
    fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let mut line_cycle = (0..self.world.road.len()).cycle();
        self.enemy.lin = line_cycle.next().unwrap();
        self.draw_road()?;
        loop {
            match getch() {
                KEY_LEFT => self.player.move_position(KEY_LEFT),
                KEY_RIGHT => self.player.move_position(KEY_RIGHT),
                KEY_F4 => break,
                _ => {}
            }
            if !self.is_alive {
                break;
            }
            self.enemy_line = line_cycle.next().unwrap();
            self.update()?;
            self.draw_road()?;
        }
        endwin();
        Ok(())
    }

    fn draw_road(&mut self) -> Result<(), Box<dyn Error>> {
        &self.world.draw_road(&mut self.player, &self.enemy)?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    initscr();
    raw();
    timeout(100);
    // allow for extended keyboard
    keypad(stdscr(), true);

    let mut screen_height = 0;
    let mut screen_width = 0;
    getmaxyx(stdscr(), &mut screen_height, &mut screen_width);

    let rng = rand::thread_rng();
    let mut game = GameState::new(rng);
    game.run()?;
    Ok(())
}
