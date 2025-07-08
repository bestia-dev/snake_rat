//! src/bin/snake_rat/main.rs
use color_eyre::Result;
use rand::Rng;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    layout::{Constraint, Layout},
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Paragraph, Wrap},
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::default().run_loop(terminal);
    ratatui::restore();
    app_result
}

struct App {
    snake_vec: Vec<(usize, usize)>,
    rat_pos: (usize, usize),
}

impl Default for App {
    fn default() -> Self {
        let snake_vec = vec![(10, 10)];
        let rat_pos = (12, 12);
        App { snake_vec, rat_pos }
    }
}

impl App {
    fn run_loop(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            return Ok(());
                        }
                        KeyCode::Down => self.move_snake(self.snake_vec[0].0, self.snake_vec[0].1 + 1),
                        // TODO: usize must never go under 0. Check it before and return some error.
                        KeyCode::Up => self.move_snake(self.snake_vec[0].0, self.snake_vec[0].1 - 1),
                        KeyCode::Left => self.move_snake(self.snake_vec[0].0 - 1, self.snake_vec[0].1),
                        KeyCode::Right => self.move_snake(self.snake_vec[0].0 + 1, self.snake_vec[0].1),
                        _ => {}
                    }
                }
            }
        }
    }

    fn draw(&self, frame: &mut Frame) {
        let frame_area = frame.area();

        let vertical = Layout::vertical([Constraint::Length(22), Constraint::Fill(1)]);
        let [content_area, instructions_area] = vertical.areas(frame_area);

        let mut text = Text::from("Press ctrl+c to quit");
        text.push_line(format!("rat: {:?}", self.rat_pos));
        text.push_line(format!("snake: {:?}", self.snake_vec));

        let paragraph = Paragraph::new(text).centered().wrap(Wrap { trim: true });
        frame.render_widget(paragraph, instructions_area);

        let horizontal = Layout::horizontal([Constraint::Length(62), Constraint::Fill(1)]);
        let [game_area, _extra_space_area] = horizontal.areas(content_area);

        let mut text = Text::default();
        for y in 0..20 {
            let mut line = Line::default();
            for x in 0..20 {
                if (x, y) == self.rat_pos {
                    line.push_span("rat");
                } else if self.snake_vec.contains(&(x, y)) {
                    line.push_span("SNK");
                } else {
                    line.push_span(" . ");
                }
            }
            text.push_line(line);
        }

        let game_content = Paragraph::new(text).block(Block::bordered().title("SNAKE-rat").on_blue());

        frame.render_widget(game_content, game_area);
    }

    fn move_snake(&mut self, nx: usize, ny: usize) {
        // out of border
        if nx >= 20 || ny >= 20 {
            panic!("out");
        }
        // crash with snake
        if self.snake_vec.contains(&(nx, ny)) {
            panic!("collision")
        }

        self.snake_vec.insert(0, (nx, ny));

        // if snake eats rat, then don't pop last element
        if self.rat_pos == (nx, ny) {
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
            // if snake don't eats rat, then pop last element
            let _popped = self.snake_vec.pop();
        }
    }
}
