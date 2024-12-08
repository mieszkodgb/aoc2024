
use crate::day6::{self, PlayerState};
use std::io;

use crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyEventKind}, terminal};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Widget},
    DefaultTerminal, Frame,
};

pub fn play_tui(raw_map: Vec<Vec<char>>) -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut App_start = App::default();
    let init_pos = day6::get_current_location(&raw_map);
    App_start.map = map_to_string(&raw_map);
    App_start.player = PlayerState{position: init_pos, step: 0, map:raw_map,history:Vec::new()};

    let app_result: Result<(), io::Error> = App_start.run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug, Default)]
pub struct App {
    map: String,
    player: PlayerState,
    exit: bool,
}

pub fn map_to_string(raw_map : &Vec<Vec<char>>) -> String{
    let map_str = raw_map
                .iter()
                .map(|row| row.iter().collect::<String>())
                .collect::<Vec<String>>()
                .join("\n");
    return  map_str;
}
impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.autoresize().unwrap();
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    /// updates the application's state based on user input
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.prev_move(),
            KeyCode::Right => self.next_move(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn next_move(&mut self) {
        self.player = day6::play_next(self.player.clone());
        self.map = map_to_string(&self.player.map);
        // self.step += 1;
    }

    fn prev_move(&mut self) {
        // self.step -= 1;
        panic!("Not done")
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Counter App Tutorial ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
            "Coord{ x: ".yellow().bold(),
            self.player.position.x.to_string().into(),
            ", y: ".yellow().bold(),
            self.player.position.y.to_string().into(),
            ", dir: ".yellow().bold(),
            self.player.position.dir.to_string().into(),
            ", step: ".yellow().bold(),
            self.player.step.to_string().into(),
            "}".into()
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);


        Paragraph::new(self.map.clone())
            .centered()
            .block(block)
            .render(area, buf);
    }
}
