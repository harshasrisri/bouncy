struct Frame {
    width: i32,
    height: i32,
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
    x: i32,
    y: i32,
    vert_dir: VertDir,
    horiz_dir: HorizDir,
}

struct Game {
    frame: Frame,
    ball: Ball,
}

impl Game {
    fn new (window: &pancurses::Window) -> Game {
        pancurses::curs_set(0);
        pancurses::noecho();
        pancurses::cbreak();

        window.clear();
        window.refresh();
        window.nodelay(true);
        window.border('|','|','-','-','+','+','+','+');

        let (height, width) = window.get_max_yx();

        Game { 
            frame: Frame { 
                width: width - 2, height: height - 2
            },
            ball: Ball {
                x: width / 2,
                y: height / 2,
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
    let mut game = Game::new(&window);

    loop {
        let (x, y) = (game.ball.y + 1, game.ball.x + 1);

        window.mvaddch(x, y, 'O');

        window.refresh();
        pancurses::napms(15);

        window.mvaddch(x, y, ' ');

        game.step();

        match window.getch() {
            Some(pancurses::Input::Character('q')) => break,
            Some(pancurses::Input::Character('Q')) => break,
            Some(pancurses::Input::KeyResize) => {
                pancurses::resize_term(0,0);
                game = Game::new(&window);
            },
            Some(_input) => (),
            None => (),
        }
    }
    window.clear();
    window.refresh();
}
