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

impl Display for Game {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        let top_bottom = |fmt: &mut Formatter| -> std::fmt::Result {
            write!(fmt, "+")?;
            for _ in 0..self.frame.width {
                write!(fmt, "-")?;
            }
            write!(fmt, "+\n")
        };

        top_bottom(fmt)?;
        for row in 0..self.frame.height {
            write!(fmt, "|")?;
            for column in 0..self.frame.width {
                let c = if row == self.ball.y  && column == self.ball.x {
                    'o'
                } else {
                    ' '
                };
                write!(fmt, "{}", c)?;
            }
            write!(fmt, "|\n")?;
        }
        top_bottom(fmt)
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
        width: (max_y - 4) as u32, 
        height: (max_x - 4) as u32,
    };
    let mut game = Game::new(frame);
    let sleep_duration = std::time::Duration::from_millis(33);

    loop {
        window.clear();
        window.printw(game.to_string());
        window.refresh();
        game.step();
        std::thread::sleep(sleep_duration);
    }
}
