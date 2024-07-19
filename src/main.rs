use std::fs;
use std::io::IsTerminal;
use std::path::PathBuf;
use std::error::Error;

use clap::{Parser, Subcommand};

pub mod num;
pub mod term;
pub mod vm;
pub mod name;
pub mod reader;
pub mod error;

use vm::Vm;
use reader::Reader;

type MainResult = Result<(), Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug, Clone)]
#[command(after_help="Run deuterostome vm program.\nA command is optional -- no command is interpreted as \"default\"")]
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

fn from_file(reader: &mut Reader, path: PathBuf) -> MainResult {
    match fs::read_to_string(path) {
        Ok(string) => term::string::exec(reader, string),
        Err(err) => Err(Box::new(err)),
    }
}

fn default(reader: &mut Reader) -> MainResult {
    if std::io::stdout().is_terminal() {
        term::readline::exec(reader)
    }
    else {
        term::line::exec(reader)
    }
}

fn main() -> MainResult {
    let cli = Cli::parse();
    let vm = &mut Vm::new();
    let result = {
        let reader = &mut Reader::new(vm);
        match cli.command() {
            Command::Default => default(reader),
            Command::File{name} => from_file(reader, name),
            Command::String{string} => term::string::exec(reader, string),
            Command::Line => term::line::exec(reader),
            Command::Term => term::readline::exec(reader),
        }
    };

    let stack = vm.stack();
    if stack.len() > 0 {
        eprintln!("Quitting with stack {:?}", stack)
    };

    result
}
