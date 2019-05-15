#[derive(Debug)]
pub struct Frame {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub enum ParseError {
    TooFewArgs,
    TooManyArgs,
    TooSmallGridSize(u32,u32),
    InvalidInteger(String),
}

struct ParseArgs(std::env::Args);

impl ParseArgs {
    fn new() -> ParseArgs {
        ParseArgs(std::env::args())
    }

    fn require_arg(&mut self) -> Result<String, ParseError> {
        match self.0.next() {
            None => Err(ParseError::TooFewArgs),
            Some(s) => Ok(s),
        }
    }

    fn require_no_args(&mut self) -> Result<(), ParseError> {
        match self.0.next() {
            Some(_) => Err(ParseError::TooManyArgs),
            None => Ok(()),
        }
    }
}

fn parse_u32(s: String) -> Result<u32, ParseError> {
    match s.parse() {
        Err(_) => Err(ParseError::InvalidInteger(s)),
        Ok(x) => Ok(x),
    }
}

pub fn parse_args() -> Result<Frame, ParseError> {
    let mut args = ParseArgs::new();

    args.require_arg()?;

    let width_str = args.require_arg()?;
    let height_str = args.require_arg()?;

    args.require_no_args()?;

    let width = parse_u32(width_str)?;
    let height = parse_u32(height_str)?;

    if width > 1 && height > 1 {
        Ok(Frame {width, height})
    } else {
        Err(ParseError::TooSmallGridSize(width, height))
    }
}

fn main () {
    println!("{:?}", parse_args());
}
