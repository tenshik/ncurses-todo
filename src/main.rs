use ncurses::*;
use std::cmp::min;

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHTED_PAIR: i16 = 1;

#[derive(Default)]
struct Ui {}

type Id = usize;

impl Ui {
    fn begin_list(&mut self, id: Id) {
        todo!()
    }
    fn list_elemet(label: &str, id: Id) {
        todo!()
    }
    fn end_list(&mut self) {
        todo!()
    }
}

fn main() {
    initscr();
    raw();
    // echo off
    noecho();
    // turn cursor invisible
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHTED_PAIR, COLOR_BLACK, COLOR_GREEN);

    let mut quit = false;
    //let mut todos: Vec<String> = Vec::new();

    let mut todos = vec![
        "Buy a bread",
        "Write the todo app",
        "Do the fucking homework",
    ];
    let done: Vec<String> = Vec::new();

    let mut current_todo: usize = 0;

    while !quit {
        for (index, todo) in todos.iter().enumerate() {
            let pair = {
                if current_todo == index {
                    HIGHLIGHTED_PAIR
                } else {
                    REGULAR_PAIR
                }
            };

            attrset(COLOR_PAIR(pair));
            mv(index as i32, 0);
            addstr(*todo);
            //attroff(COLOR_PAIR(pair));
        }

        refresh();

        let key = getch();

        match key as u8 as char {
            'q' => quit = true,
            'k' => {
                if current_todo > 0 {
                    current_todo -= 1;
                }
            }
            'j' => current_todo = min(current_todo + 1, todos.len() - 1),
            _ => {}
        }
    }
    getch();
    endwin();
}
