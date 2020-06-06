use std::{error::Error, thread, time};

use ncurses::*;
use rand::prelude::*;

struct Player<'b> {
    col: i32,
    lin: i32,
    is_alive: bool,
    repr: &'b str,
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
    col: i32,
    lin: i32,
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
        line: i32,
        player: &mut Player,
        enemy: &Enemy,
    ) -> Result<String, Box<dyn Error>> {
        let row = field
            .iter()
            .enumerate()
            .map(|(f, n)| {
                if ((line as i32) == player.lin)
                    && ((f as i32) == player.col)
                    && (player.col >= 1 && player.col <= 3)
                {
                    player.repr
                } else if line as i32 == enemy.lin && f as i32 == enemy.col {
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
            mvaddstr(
                i as i32,
                0,
                &self.draw_field(field, i as i32, player, enemy)?,
            );
        }
        Ok(())
    }
}

struct GameState<'a, 'b, 'c> {
    is_alive: bool,
    score: i32,
    enemy_line: i32,
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
        if !self.player.is_alive {
            self.is_alive = false;
        }
        if self.enemy_line == 10 && self.player.is_alive {
            self.score += 1
        }
        mvprintw(LINES() - 1, 0, &format!("score: {}", self.score));
        Ok(())
    }
    fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let mut line_cycle = (0..self.world.road.len()).cycle();
        self.enemy.lin = line_cycle.next().unwrap() as i32;
        self.draw_road()?;
        loop {
            match getch() {
                KEY_LEFT => {
                    self.player.col = if self.player.col <= 1 {
                        1
                    } else {
                        self.player.col - 1
                    }
                }
                KEY_RIGHT => {
                    self.player.col = if self.player.col >= 3 {
                        3
                    } else {
                        self.player.col + 1
                    }
                }
                KEY_F4 => break,
                _ => {}
            }
            if !self.is_alive {
                break;
            }
            self.enemy_line = line_cycle.next().unwrap() as i32;
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
    timeout(50);
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
