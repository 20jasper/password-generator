use core::iter;

use rand::distributions::{Distribution, Slice, Uniform};
use rand::prelude::Rng;
use ratatui::{
    prelude::{Frame, *},
    widgets::{block::Title, Paragraph, Wrap},
};

use crate::app::{styled_block, App};

use super::PasswordType;

// I excluded whitespace characters and the backtick comma, and period, and quote, and backslash. [original list](https://owasp.org/www-community/password-special-characters)
const SYMBOLS_ARR: [char; 25] = [
    '!', '#', '$', '%', '&', '(', ')', '*', '+', '-', '/', ':', ';', '<', '=', '>', '?', '@', '[',
    ']', '^', '_', '{', '|', '}',
];

impl PasswordType {
    pub fn generate(&self, length: usize) -> String {
        match self {
            PasswordType::Pin => get_random_string(length, true, false, false),
            PasswordType::Random {
                numbers, symbols, ..
            } => get_random_string(length, *numbers, *symbols, true),
        }
    }
}

enum DistributionType<'a> {
    Uniform(Uniform<char>),
    Slice(Slice<'a, char>),
}

impl DistributionType<'_> {
    fn sample(&self, rng: &mut impl Rng) -> char {
        match self {
            DistributionType::Uniform(dist) => dist.sample(rng),
            DistributionType::Slice(dist) => *dist.sample(rng),
        }
    }
}

fn get_distributions<'a>(numbers: bool, symbols: bool, letters: bool) -> Vec<DistributionType<'a>> {
    use DistributionType as D;

    let mut ranges = vec![];

    if numbers {
        ranges.push(D::Uniform(Uniform::from('0'..='9')));
    }
    if letters {
        ranges.push(D::Uniform(Uniform::from('a'..='z')));
        ranges.push(D::Uniform(Uniform::from('A'..='Z')));
    }
    if symbols {
        let dist = Slice::new(&SYMBOLS_ARR).expect("slice shouldn't be empty");
        ranges.push(D::Slice(dist));
    }

    ranges
}

/// generates a random string of length `length` with at least one character
/// from each of the options marked true
fn get_random_string(length: usize, numbers: bool, symbols: bool, letters: bool) -> String {
    let distributions = get_distributions(numbers, symbols, letters);

    (0..distributions.len())
        .chain(iter::from_fn(|| {
            Some(rand::thread_rng().gen_range(0..distributions.len()))
        }))
        .map(|i| distributions[i].sample(&mut rand::thread_rng()))
        .take(length)
        .collect::<String>()
}

pub fn render(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let title = Title::from(" Password Generator ".bold());
    let instructions = Title::from(Line::from(vec![
        " Decrease Length ".into(),
        "<Left>/<H>".blue().bold(),
        " Increase Length ".into(),
        "<Right>/<L>".blue().bold(),
        " Copy ".into(),
        "<C>/<Y>".blue().bold(),
        " Back to List ".into(),
        "<Q> ".blue().bold(),
    ]));
    let block = styled_block(title, instructions);

    let password_text = Paragraph::new(vec![
        Line::from(vec!["Length: ".into(), app.length.to_string().yellow()]),
        Line::from(vec!["Password: ".into(), app.password.clone().yellow()]),
    ])
    .wrap(Wrap { trim: true })
    .block(block);

    frame.render_widget(password_text, area);
}
