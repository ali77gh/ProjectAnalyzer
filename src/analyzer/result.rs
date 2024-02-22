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

    pub fn post_set(&self) -> &HashSet<String> {
        &self.post_set
    }
}

impl ToString for AnalyzeResult {
    fn to_string(&self) -> String {
        let mut result = String::new();

        result.push_str("┌───────────────────────────────────────────────┐\n");
        result.push_str("│                ProjectAnalyzer                │\n");
        result.push_str("│                                               │\n");
        result.push_str("│ https://github.com/ali77gh/ProjectAnalyzer    │\n");
        result.push_str("│                                               │\n");
        for postfix in self.post_set.iter() {
            let file_counter = self.file_counter.get(postfix).unwrap();
            let line_counter = self.line_counter.get(postfix).unwrap();
            let division = if file_counter == &0 {
                0
            } else {
                line_counter / file_counter
            };
            result.push_str("├───────────────────────────────────────────────┤\n");
            result
                .push_str(format!("│ {} files result:                 \t\t│\n", postfix).as_str());
            result.push_str(
                format!(
                    "│   {} {} files                   \t\t│\n",
                    file_counter, postfix,
                )
                .as_str(),
            );
            result.push_str(
                format!(
                    "│   {} lines of {}               \t\t│\n",
                    line_counter, postfix
                )
                .as_str(),
            );
            result.push_str(
                format!("│   average lines per file: {}       \t\t│\n", division).as_str(),
            );
        }
        result.push_str("└───────────────────────────────────────────────┘\n");
        result
    }
}
