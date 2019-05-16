use std::fmt::{Display, Formatter};

struct Frame {
    width: u32,
    height: u32,
}

enum VertDir {
    Up,
    Down,
}

enum HorizDir {
    Left,
    Right,
}

struct Ball {
    x: u32,
    y: u32,
    vert_dir: VertDir,
    horiz_dir: HorizDir,
}

struct Game {
    frame: Frame,
    ball: Ball,
}

impl Game {
    fn new(frame: Frame) -> Game {
        Game {
            frame,
            ball: Ball {
                x: 2,
                y: 4,
                vert_dir: VertDir::Up,
                horiz_dir: HorizDir::Left,
            }
        }
    }

    fn step (&mut self) {
        self.ball.bounce(&self.frame);
        self.ball.mv();
    }
}

impl Ball {
    fn bounce(&mut self, frame: &Frame) {
        if self.x == 0 {
            self.horiz_dir = HorizDir::Right;
        } if self.x == frame.width - 1 {
            self.horiz_dir = HorizDir::Left;
        } 
        if self.y == frame.height - 1 {
            self.vert_dir = VertDir::Up;
        } else if self.y == 0 {
            self.vert_dir = VertDir::Down;
        }
    }

    fn mv(&mut self) {
        match self.horiz_dir {
            HorizDir::Left => self.x -= 1,
            HorizDir::Right => self.x += 1,
        };
        match self.vert_dir {
            VertDir::Up => self.y -= 1,
            VertDir::Down => self.y += 1,
        };
    }
}

fn main() {
    let window = pancurses::initscr();
    let (max_x, max_y) = window.get_max_yx();
    let frame = Frame {
        width: (max_y - 2) as u32, 
        height: (max_x - 2) as u32,
    };
    let mut game = Game::new(frame);

    pancurses::curs_set(0);
    pancurses::noecho();
    pancurses::cbreak();

    window.border('|','|','-','-','+','+','+','+');
    window.nodelay(true);

    loop {
        let (x, y) = ((game.ball.y + 1) as i32, (game.ball.x + 1) as i32);

        window.mv(x,y);
        window.addch('o');

        window.refresh();
        pancurses::napms(30);

        window.mv(x,y);
        window.addch(' ');

        game.step();

        match window.getch() {
            Some(pancurses::Input::Character('q')) => break,
            Some(pancurses::Input::Character('Q')) => break,
            Some(input) => (),
            None => (),
        }
    }
    window.clear();
    window.refresh();
}
