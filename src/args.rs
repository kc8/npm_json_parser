use clap::Parser;

pub struct ArgsConfig {
    pub file_path: String,
    pub ignore_case: bool,
}

#[derive(Parser)]
#[command(
    author = "kyle",
    version = "0.0.1-beta",
    about = "package.json script checker"
)]
pub struct Cli {
    /// Relative or full file path to the package.json file
    #[arg(value_name = "FILE")]
    file_path: String,

    /// ignore case in JSON scripts
    #[clap(short = 'c', long, default_value = "true")]
    ignore_case: bool,
}

impl Cli {
    pub fn get_args() -> ArgsConfig {
        let cli = Cli::parse();
        ArgsConfig { file_path: cli.file_path, ignore_case: cli.ignore_case }
    }
}
