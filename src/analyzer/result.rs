pub struct AnalyzeResultItem {
    postfix: String,
    name: Option<String>, // from json database
    files: u64,
    lines: u64,
    empty_lines: u64, // TODO
}

impl AnalyzeResultItem {
    pub fn new(postfix: String, files: u64, lines: u64) -> Self {
        Self {
            postfix,
            name: None, // TODO
            files,
            lines,
            empty_lines: 0, // TODO
        }
    }

    pub fn empty_lines(&self) -> u64 {
        self.empty_lines
    }
    pub fn lines(&self) -> u64 {
        self.lines
    }
    pub fn files(&self) -> u64 {
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
    pub fn new(vec: Vec<AnalyzeResultItem>) -> Self {
        Self(vec)
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
