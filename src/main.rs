use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::Print,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use std::{
    io::{self, Write},
    str::FromStr,
    time::{Duration, Instant},
};

const WIDTH: u16 = 40;
const HEIGHT: u16 = 20;
const TICK_RATE: Duration = Duration::from_millis(200);

struct Position {
    x: u16,
    y: u16,
}

#[derive(PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self, o: &Direction) -> bool {
        let opposite_direction = match self {
            Direction::Down => Direction::Up,
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        };
        return *o == opposite_direction;
    }
}

struct Snake {
    body: Vec<Position>,
    direction: Direction,
}

impl Snake {
    fn new() -> Self {
        let x = WIDTH / 2;
        let y = HEIGHT / 2;
        Snake {
            body: vec![
                Position { x: x + 1, y: y },
                Position { x: x, y: y },
                Position { x: x - 1, y: y },
            ],
            direction: Direction::Right,
        }
    }

    fn change_direction(&mut self, new_direction: Direction) {
        if !self.direction.opposite(&new_direction) {
            self.direction = new_direction;
        }
    }

    fn slither(&mut self) {
        for i in (1..self.body.len()).rev() {
            self.body[i].x = self.body[i - 1].x;
            self.body[i].y = self.body[i - 1].y;
        }
        match self.direction {
            Direction::Down => self.body[0].y += 1,
            Direction::Left => self.body[0].x -= 1,
            Direction::Right => self.body[0].x += 1,
            Direction::Up => self.body[0].y -= 1,
        }
    }

    fn is_out(&self) -> bool {
        for pos in self.body.iter() {
            if pos.x <= 0 || pos.y <= 0 || pos.x >= WIDTH || pos.y >= HEIGHT {
                return true;
            }
        }
        return false;
    }
}

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let mut snake = Snake::new();

    enable_raw_mode()?;
    cls(&mut stdout)?;

    draw_board(&mut stdout)?;
    draw_snake(&mut stdout, &mut snake)?;

    let mut last_tick = Instant::now();

    loop {
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Up => snake.change_direction(Direction::Up),
                    KeyCode::Down => snake.change_direction(Direction::Down),
                    KeyCode::Left => snake.change_direction(Direction::Left),
                    KeyCode::Right => snake.change_direction(Direction::Right),
                    KeyCode::Backspace => break,
                    _ => {}
                };
            }
        }
        if last_tick.elapsed() >= TICK_RATE {
            snake.slither();
            cls(&mut stdout)?;
            draw_board(&mut stdout)?;
            draw_snake(&mut stdout, &mut snake)?;
            stdout.flush()?;
            if snake.is_out() {
                you_lost(&mut stdout)?;
                std::thread::sleep(Duration::from_secs(5));
                break;
            }
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    execute!(stdout, Show)?;

    Ok(())
}

fn draw_board(stdout: &mut io::Stdout) -> io::Result<()> {
    // draw borders
    execute!(stdout, MoveTo(0, 0))?;

    // draw top line
    execute!(stdout, Print("┌"))?;
    for _ in 0..WIDTH {
        execute!(stdout, Print("─"))?;
    }
    execute!(stdout, Print("┐"))?;

    // draw side lines
    for y in 1..HEIGHT {
        execute!(stdout, MoveTo(0, y), Print("│"))?;
        execute!(stdout, MoveTo(WIDTH + 1, y), Print("│"))?;
    }

    // draw bottom line
    execute!(stdout, MoveTo(0, HEIGHT), Print("└"))?;
    for _ in 0..WIDTH {
        execute!(stdout, Print("─"))?;
    }
    execute!(stdout, Print("┘"))?;

    stdout.flush()?;
    Ok(())
}

fn draw_snake(stdout: &mut io::Stdout, snake: &mut Snake) -> io::Result<()> {
    for (_, pos) in snake.body.iter().enumerate() {
        execute!(stdout, MoveTo(pos.x, pos.y), Print("▄"))?;
    }
    stdout.flush()?;
    Ok(())
}

fn cls(stdout: &mut io::Stdout) -> io::Result<()> {
    execute!(stdout, Clear(ClearType::All), Hide)?;
    Ok(())
}

fn you_lost(stdout: &mut io::Stdout) -> io::Result<()> {
    cls(stdout)?;
    draw_board(stdout)?;
    let msg: String = String::from_str("You lost").unwrap();
    execute!(
        stdout,
        MoveTo(WIDTH / 2 - (msg.len() as u16) / 2, HEIGHT / 2),
        Print(msg)
    )?;
    Ok(())
}
