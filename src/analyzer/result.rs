pub struct AnalyzeResultItem {
    postfix: String,
    name: Option<String>, // from json database
    files: usize,
    lines: usize,
    empty_lines: usize, // TODO
}

impl AnalyzeResultItem {
    pub fn new(postfix: String, files: usize, lines: usize) -> Self {
        Self {
            postfix,
            name: None, // TODO
            files,
            lines,
            empty_lines: 0, // TODO
        }
    }

    pub fn empty_lines(&self) -> usize {
        self.empty_lines
    }
    pub fn lines(&self) -> usize {
        self.lines
    }
    pub fn files(&self) -> usize {
        self.files
    }
    pub fn name(&self) -> &str {
        match &self.name {
            Some(x) => x.as_str(),
            None => self.postfix.as_str(),
        }
    }
}

pub struct AnalyzeResult(Vec<AnalyzeResultItem>);

impl AnalyzeResult {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn add(&mut self, postfix: &str, content: Vec<u8>) {
        let position = self.0.iter().position(|x| x.postfix == postfix);
        let lines = content.iter().filter(|i| **i == b'\n').count();

        match position {
            Some(position) => {
                self.0[position].files += 1;
                self.0[position].lines += lines;
            }
            None => self
                .0
                .push(AnalyzeResultItem::new(postfix.to_string(), 1, lines)),
        }
    }

    pub fn iter(&self) -> &Vec<AnalyzeResultItem> {
        &self.0
    }
}

impl ToString for AnalyzeResult {
    fn to_string(&self) -> String {
        let mut table = crate::ui::table::Table::new();

        table.write_center("ProjectAnalyzer");

        table.empty_line();
        table.write("https://github.com/ali77gh/ProjectAnalyzer");
        table.empty_line();

        for item in self.iter() {
            if item.lines == 0 {
                continue;
            }
            table.draw_line();
            table.write(format!("{} files result:", item.name()));
            table.write(format!("  files: {}", item.files()));
            table.write(format!("  lines: {} ", item.lines()));
        }
        table.render_table()
    }
}
