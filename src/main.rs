//! src/bin/snake_rat/main.rs

use color_eyre::Result;
use rand::Rng;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers, poll},
    layout::{Constraint, Layout},
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Paragraph},
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::default().app_loop(terminal);
    ratatui::restore();
    app_result
}

/// Application state
struct App {
    snake_vec: Vec<(usize, usize)>,
    rat_pos: (usize, usize),
    last_direction: Direction,
    dinner: bool,
    points: u32,
    timer: u32,
    dead: bool,
}

/// Initial app state
impl Default for App {
    fn default() -> Self {
        let snake_vec = vec![(10, 10)];
        let rat_pos = (12, 12);
        App {
            snake_vec,
            rat_pos,
            last_direction: Direction::Up,
            dinner: false,
            points: 0,
            timer: 0,
            dead: false,
        }
    }
}

/// The last direction
#[derive(Clone)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl App {
    /// Ratatui app is a loop with 2 functions: draw and react events
    fn app_loop(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if poll(std::time::Duration::from_millis(200))? {
                // It's guaranteed that `read` won't block, because `poll` returned
                // `Ok(true)`.
                // react on events
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => return Ok(()),
                            KeyCode::Char('q') => return Ok(()),

                            KeyCode::Char('n') => self.restart_game(),

                            KeyCode::Down => self.move_snake(Direction::Down),
                            KeyCode::Up => self.move_snake(Direction::Up),
                            KeyCode::Left => self.move_snake(Direction::Left),
                            KeyCode::Right => self.move_snake(Direction::Right),
                            _ => {}
                        }
                    }
                }
            } else {
                // Timeout expired, no `Event` is available
                if !self.dead {
                    self.move_snake(self.last_direction.clone());
                }
            }
        }
    }

    fn draw(&self, frame: &mut Frame) {
        let frame_area = frame.area();

        let horizontal = Layout::horizontal([Constraint::Length(62), Constraint::Fill(1)]);
        let [content_area, _extra_horizontal_area] = horizontal.areas(frame_area);

        let vertical = Layout::vertical([
            Constraint::Length(22),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ]);
        let [game_area, instructions_area, dead_area, _extra_vertical_ares] = vertical.areas(content_area);

        let mut text = Text::default();
        for y in 0..20 {
            let mut line = Line::default();
            for x in 0..20 {
                if (x, y) == self.rat_pos {
                    line.push_span("rat");
                } else if self.snake_vec.contains(&(x, y)) {
                    if self.dinner {
                        line.push_span("NAM");
                    } else if self.dead {
                        line.push_span("DEA");
                    } else {
                        line.push_span("SNK");
                    }
                } else {
                    line.push_span(" . ");
                }
            }
            text.push_line(line);
        }

        let game_content = Paragraph::new(text).block(Block::bordered().title("SNAKE-rat").on_blue());
        frame.render_widget(game_content, game_area);

        let horizontal_instructions = Layout::horizontal([Constraint::Ratio(1, 3), Constraint::Ratio(1, 3), Constraint::Ratio(1, 3)]);
        let [time_area, points_area, exit_area] = horizontal_instructions.areas(instructions_area);

        let paragraph = Paragraph::new(format!("time: {}", self.timer));
        frame.render_widget(paragraph, time_area);

        let paragraph = Paragraph::new(format!("points: {}", self.points));
        frame.render_widget(paragraph, points_area);

        let paragraph = Paragraph::new("Press Q to quit");
        frame.render_widget(paragraph, exit_area);

        if self.dead {
            let paragraph = Paragraph::new("The snake is dead! Press N to restart.");
            frame.render_widget(paragraph, dead_area);
        }
    }

    fn move_snake(&mut self, direction: Direction) {
        if !self.dead {
            self.timer += 1;
            self.last_direction = direction;

            let (mut nx, mut ny) = self.snake_vec[0];
            // dead if out of border
            match self.last_direction {
                Direction::Up => {
                    if ny == 0 {
                        self.dead = true;
                    } else {
                        ny -= 1;
                    }
                }
                Direction::Down => {
                    if ny == 19 {
                        self.dead = true;
                    } else {
                        ny += 1;
                    }
                }
                Direction::Left => {
                    if nx == 0 {
                        self.dead = true;
                    } else {
                        nx -= 1;
                    }
                }
                Direction::Right => {
                    if nx == 19 {
                        self.dead = true;
                    } else {
                        nx += 1;
                    }
                }
            }
            if !self.dead {
                // crash with snake
                if self.snake_vec.contains(&(nx, ny)) {
                    self.dead = true;
                }

                self.snake_vec.insert(0, (nx, ny));

                // if snake eats rat, then don't pop last element
                if self.rat_pos == (nx, ny) {
                    self.dinner = true;
                    self.points += 1;
                    // create new random rat away from the snake
                    let mut rng = rand::rng();
                    loop {
                        let rx = rng.random_range(0..20);
                        let ry = rng.random_range(0..20);
                        if self.snake_vec.contains(&(rx, ry)) {
                            // continue loop
                            continue;
                        }
                        self.rat_pos = (rx, ry);
                        break;
                    }
                } else {
                    self.dinner = false;
                    // if snake don't eats rat, then pop last element
                    let _popped = self.snake_vec.pop();
                }
            }
        }
    }

    fn restart_game(&mut self) {
        *self = Self::default();
    }
}
