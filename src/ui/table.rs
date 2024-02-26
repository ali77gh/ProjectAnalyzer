#[derive(Clone)]
pub struct Table {
    content: Vec<InTableContent>,
}

impl Table {
    pub fn new() -> Self {
        Self {
            content: vec![InTableContent::TopLine],
        }
    }

    pub fn write(&mut self, line: impl Into<String>) {
        self.content.push(InTableContent::Text(line.into()));
    }
    pub fn write_center(&mut self, line: impl Into<String>) {
        self.content.push(InTableContent::CenterText(line.into()));
    }
    pub fn empty_line(&mut self) {
        self.content.push(InTableContent::Text("".to_string()));
    }
    pub fn draw_line(&mut self) {
        self.content.push(InTableContent::MiddleLine);
    }

    pub fn render_table(mut self) -> String {
        self.content.push(InTableContent::BottomLine);

        let width = self
            .content
            .iter()
            .filter_map(|x| match x {
                InTableContent::CenterText(s) => Some(s.len()),
                InTableContent::Text(s) => Some(s.len()),
                _ => None,
            })
            .max();

        let width = width.unwrap_or(1) + 1; //+1 is for left margin

        self.content
            .iter()
            .map(|content| content.render(width))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[derive(Clone)]
enum InTableContent {
    TopLine,
    CenterText(String),
    Text(String),
    MiddleLine,
    BottomLine,
}

impl InTableContent {
    pub fn render(&self, width: usize) -> String {
        match self {
            InTableContent::TopLine => format!("┌{}┐", "─".repeat(width + 1)),
            InTableContent::MiddleLine => format!("├{}┤", "─".repeat(width + 1)),
            InTableContent::BottomLine => format!("└{}┘", "─".repeat(width + 1)),
            InTableContent::Text(text) => format!("│ {}{}│", text, " ".repeat(width - text.len())),
            InTableContent::CenterText(text) => {
                let space_around = " ".repeat((width - text.len()) / 2);
                let fix = if text.len() % 2 == 0 { " " } else { "" };
                format!("│ {}{}{}{}│", space_around, text, space_around, fix)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_test() {
        let mut table = Table::new();
        table.write_center("Rust");
        table.draw_line();
        table.write("Hello my name is Sam");
        table.empty_line();
        let rendered = table.render_table();
        println!("{}", &rendered);
        let expected = "┌──────────────────────┐
│         Rust         │
├──────────────────────┤
│ Hello my name is Sam │
│                      │
└──────────────────────┘";
        assert_eq!(rendered, expected);
    }
}
