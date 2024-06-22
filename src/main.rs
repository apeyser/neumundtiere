use std::fs;
use std::io::IsTerminal;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

pub mod term;
pub mod rpn;
pub mod reader;

use rpn::Vm;
use reader::Reader;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug, Clone)]
#[command(after_help="Run rpn program.\nA command is optional -- no command is interpreted as \"default\"")]
enum Command {
    /// Interpret a file
    File {
        /// Name of file to interpret
        name: PathBuf
    },
    /// Interpret a string
    String {
        /// String to interpret
        string: String
    },
    /// Run in line mode
    Line,
    /// Run in term mode
    Term,
    /// Run in term mode if possible; otherwise line mode
    Default,
}

impl Cli {
    pub fn command(&self) -> Command {
        self.command.clone().unwrap_or(Command::Default)
    }
}

fn main() {
    let cli = Cli::parse();
    let vm = &mut Vm::new();
    let reader = &Reader::new();

    match cli.command() {
        Command::Default => {
            if std::io::stdout().is_terminal() {term::readline::exec(vm, reader)}
            else {term::line::exec(vm, reader)}
        },
        Command::File{name} => {
            match fs::read_to_string(name) {
                Err(err) => println!("File error: {err:?}"),
                Ok(string) => term::string::exec(vm, reader, string),
            }
        },
        Command::String{string} => term::string::exec(vm, reader, string),
        Command::Line => term::line::exec(vm, reader),
        Command::Term => term::readline::exec(vm, reader),
    }
}
