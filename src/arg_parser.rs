use clap::{Parser, Subcommand};

/// Fast Rust binary that counts line numbers of a codebase
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct MyArgs {
    /// Number of times to greet
    #[arg(short, long, default_value_t = String::from("."))]
    root_dir: String,

    /// Example: project_analyzer --ignore node_modules --ignore dist
    #[arg(long)]
    ignore: Vec<String>,

    /// Will keep running and update result whenever anything changed.
    #[arg(short, long)]
    watch: bool,

    /// Filter by list of file postfixes example: project_analyzer --postfixes py,rs,cpp
    #[arg(short = 'p', long)]
    postfixes: Option<String>,

    /// Output as json
    #[arg(short, long, default_value_t = false)]
    json: bool,

    #[command(subcommand)]
    command: Option<MyCommands>,
}

impl MyArgs {
    pub fn root_dir(&self) -> &str {
        &self.root_dir
    }

    pub fn ignore(&self) -> &[String] {
        &self.ignore
    }

    pub fn postfixes(&self) -> Option<Vec<String>> {
        self.postfixes.clone().map(|s| {
            s.split(',')
                .filter(|i| !i.is_empty())
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
        })
    }

    pub fn command(&self) -> Option<&MyCommands> {
        self.command.as_ref()
    }

    pub fn json(&self) -> bool {
        self.json
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum MyCommands {
    /// Opens github
    Update,
    Watch, //TODO remove this
}
