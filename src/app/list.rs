use core::fmt::Display;
use std::string::ToString;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
    widgets::{block::Title, List, ListItem, ListState},
};

use super::styled_block;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Items<T> {
    pub items: Vec<T>,
    pub state: ListState,
}

impl<T> Items<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self {
            items,
            state: ListState::default().with_selected(Some(0)),
        }
    }

    pub fn select(&mut self, index: usize) {
        self.state.select(Some(index));
    }

    pub fn get_selected(&self) -> Option<&T> {
        Some(&self.items[self.state.selected()?])
    }

    pub fn next(&mut self) {
        let next = self
            .state
            .selected()
            .map(|x| x.saturating_add(1) % self.items.len());

        self.state.select(next);
    }

    pub fn previous(&mut self) {
        let previous = self.state.selected().map(|x| {
            x.wrapping_sub(1)
                .min(self.items.len() - 1)
        });
        self.state.select(previous);
    }

    pub fn handle_key_event(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('j') | KeyCode::Down => {
                self.next();
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.previous();
            }
            _ => {}
        }
    }
}

pub fn ui(frame: &mut Frame<'_>, items: &mut Items<impl Display>) {
    let area = frame.size();
    render(frame, area, items);
}

pub fn styled<'a, T>(title: &'a str, instructions: Title<'a>, items: T) -> List<'a>
where
    T: IntoIterator,
    T::Item: Into<ListItem<'a>>,
{
    let title = Title::from(title.bold());
    let block = styled_block(title, instructions);

    List::new(items)
        .block(block)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .highlight_symbol(">> ")
        .repeat_highlight_symbol(true)
}

pub fn render(frame: &mut Frame<'_>, area: Rect, items: &mut Items<impl Display>) {
    let instructions = Title::from(Line::from(vec![
        " Next ".into(),
        "<Down>/<J>".blue().bold(),
        " Last ".into(),
        "<Up>/<K>".blue().bold(),
        " Select ".into(),
        "<Enter>/<Space>".blue().bold(),
        " Quit ".into(),
        "<Q> ".blue().bold(),
    ]));
    let list = styled(
        " Password Types ",
        instructions,
        items
            .items
            .iter()
            .map(ToString::to_string),
    );

    frame.render_stateful_widget(list, area, &mut items.state);
}
