use std::collections::{HashMap, HashSet};

pub struct AnalyzeResult {
    file_counter: HashMap<String, u64>,
    line_counter: HashMap<String, u64>,
    post_set: HashSet<String>,
}

impl AnalyzeResult {
    pub fn new(
        file_counter: HashMap<String, u64>,
        line_counter: HashMap<String, u64>,
        post_set: HashSet<String>,
    ) -> Self {
        Self {
            file_counter,
            line_counter,
            post_set,
        }
    }

    pub fn file_counter(&self) -> &HashMap<String, u64> {
        &self.file_counter
    }

    pub fn line_counter(&self) -> &HashMap<String, u64> {
        &self.line_counter
    }
}

impl ToString for AnalyzeResult {
    fn to_string(&self) -> String {
        let mut table = crate::ui::table::Table::new();

        table.write_center("ProjectAnalyzer");

        table.empty_line();
        table.write("https://github.com/ali77gh/ProjectAnalyzer");
        table.empty_line();

        for postfix in self.post_set.iter() {
            let file_counter = self.file_counter.get(postfix).unwrap();
            let line_counter = self.line_counter.get(postfix).unwrap();
            if *line_counter == 0 {
                continue;
            }
            table.draw_line();
            table.write(format!("{} files result:", postfix));
            table.write(format!("  {} {} files", file_counter, postfix));
            table.write(format!("  {} lines of {} ", line_counter, postfix));
        }
        table.render_table()
    }
}
