use std::{thread, time};

use ncurses::*;
use rand::prelude::*;

fn draw_field(
    field: &Vec<&str>,
    line: i32,
    user_position: i32,
    enemy_line_position: i32,
    enemy_column_position: i32,
) -> String {
    // magic numbers!
    let default_car_line = 9;
    field
        .iter()
        .enumerate()
        .map(|(f, n)| {
            if ((line as i32) == default_car_line)
                && ((f as i32) == user_position)
                && (user_position >= 1 && user_position <= 3)
            {
                "C"
            } else if line as i32 == enemy_line_position && f as i32 == enemy_column_position {
                "Z"
            } else if enemy_line_position == default_car_line
                && enemy_column_position == user_position
            {
                "X"
            } else {
                n
            }
        })
        .collect::<Vec<&str>>()
        .join("")
}

fn draw_road(
    road: &Vec<Vec<&str>>,
    user_car: i32,
    enemy_line_position: i32,
    enemy_column_position: i32,
) {
    for (i, field) in road.iter().enumerate() {
        mvaddstr(
            i as i32,
            0,
            &draw_field(
                field,
                i as i32,
                user_car,
                enemy_line_position,
                enemy_column_position,
            ),
        );
    }
    // for debug prupose
    mvprintw(LINES() - 2, 0, &format!("current user: {}", user_car));
}

fn main() {
    // initialize ncurses
    initscr();
    raw();
    timeout(50);

    let mut screen_height = 0;
    let mut screen_width = 0;
    getmaxyx(stdscr(), &mut screen_height, &mut screen_width);

    // allow for extended keyboard
    keypad(stdscr(), true);

    mvprintw(LINES() - 4, 0, "use arrow keys left and right to move");
    mvprintw(LINES() - 3, 0, "Press F4 to exit");

    let mut current_user: i32 = 2;
    let road: Vec<Vec<&str>> = vec![
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
    ];
    let mut enemy_cycle_line = (0..road.len()).cycle();
    let mut rng = rand::thread_rng();
    let mut enemy_column_position = rng.gen_range(1, 4);
    draw_road(
        &road,
        current_user,
        enemy_cycle_line.next().unwrap() as i32,
        enemy_column_position,
    );
    let ten_millis = time::Duration::from_millis(10);

    let mut ch = getch();
    let mut change_column: bool = false;
    while ch != KEY_F(4) {
        if change_column {
            enemy_column_position = rng.gen_range(1, 4);
            change_column = false;
        }
        match ch {
            KEY_LEFT => {
                current_user = if current_user <= 1 {
                    1
                } else {
                    current_user - 1
                }
            }
            KEY_RIGHT => {
                current_user = if current_user >= 3 {
                    3
                } else {
                    current_user + 1
                }
            }
            _ => {}
        }
        let line = enemy_cycle_line.next().unwrap() as i32;
        if line == 10 {
            change_column = true;
        };

        draw_road(&road, current_user, line, enemy_column_position);
        ch = getch();
        thread::sleep(ten_millis);
    }
    // finish curses
    endwin();
}
