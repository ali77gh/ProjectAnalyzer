use piechart::{Chart, Color, Data, Style};

use crate::analyzer::result::AnalyzeResult;

pub trait DrawableChart {
    fn draw(&self);
}

impl DrawableChart for AnalyzeResult {
    fn draw(&self) {
        if self.iter().len() <= 1 {
            return;
        }

        let mut dataset = vec![];
        let mut counter = 0;
        for item in self.iter() {
            let style: Style = (get_rounded(&COLORS, counter)).into();
            dataset.push(Data {
                label: item.postfix().to_string(),
                value: item.lines() as f32,
                color: Some(style),
                fill: get_rounded(&CHARS, counter),
            });
            counter += 1;
        }
        if counter > 10 {
            counter = 10;
        }
        // draw chart
        Chart::new()
            .radius(counter as u16 + 4)
            .aspect_ratio(3)
            .legend(true)
            .draw(&dataset);
    }
}

fn get_rounded<T: Clone>(array: &[T], index: usize) -> T {
    let mut index = index;
    while index >= array.len() {
        index -= array.len();
    }
    array[index].clone()
}
const CHARS: [char; 7] = ['▰', '▼', '◆', '■', '•', '▪', '▴'];

const COLORS: [Color; 7] = [
    Color::Green,
    Color::Yellow,
    Color::Blue,
    Color::Purple,
    Color::Cyan,
    Color::White,
    Color::Red,
];
