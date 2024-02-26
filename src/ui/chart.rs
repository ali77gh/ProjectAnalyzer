use piechart::{Chart, Color, Data, Style};

use crate::analyzer::result::AnalyzeResult;

pub trait DrawableChart {
    fn draw(&self);
}

impl DrawableChart for AnalyzeResult {
    fn draw(&self) {
        let mut dataset = vec![];
        let mut counter = 0;
        for item in self.iter() {
            let style: Style = (*COLORS.get(counter).unwrap()).into();
            dataset.push(Data {
                label: item.name().to_string(),
                value: item.lines() as f32,
                color: Some(style),
                fill: *CHARS.get(counter).unwrap(),
            });
            counter += 1;
        }
        // draw chart
        Chart::new()
            .radius(counter as u16 + 4)
            .aspect_ratio(3)
            .legend(true)
            .draw(&dataset);
    }
}

const CHARS: [char; 28] = [
    '▰', '▼', '◆', '■', '•', '▪', '▴', '▰', '▼', '◆', '■', '•', '▪', '▴', '▰', '▼', '◆', '■', '•',
    '▪', '▴', '▰', '▼', '◆', '■', '•', '▪', '▴',
];

const COLORS: [Color; 18] = [
    Color::Green,
    Color::Yellow,
    Color::Blue,
    Color::Purple,
    Color::Cyan,
    Color::White,
    Color::Red,
    Color::Green,
    Color::Yellow,
    Color::Blue,
    Color::Purple,
    Color::Cyan,
    Color::White,
    Color::Red,
    Color::Green,
    Color::Yellow,
    Color::Blue,
    Color::Purple,
];
