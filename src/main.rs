use std::fs;
use std::io::IsTerminal;
use std::path::PathBuf;
use std::error::Error;

use clap::{Parser, Subcommand};

pub mod term;
pub mod rpn;
pub mod reader;

use rpn::Vm;
use reader::Reader;

type MainResult = Result<(), Box<dyn Error>>;

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

fn from_file(vm: &mut Vm, reader: &Reader, path: PathBuf) -> MainResult {
    match fs::read_to_string(path) {
        Ok(string) => term::string::exec(vm, reader, string),
        Err(err) => Err(Box::new(err)),
    }
}

fn default(vm: &mut Vm, reader: &Reader) -> MainResult {
    if std::io::stdout().is_terminal() {
        term::readline::exec(vm, reader)
    }
    else {
        term::line::exec(vm, reader)
    }
}

fn main() -> MainResult {
    let cli = Cli::parse();
    let vm = &mut Vm::new();
    let reader = &Reader::new();

    let result = match cli.command() {
        Command::Default => default(vm, reader),
        Command::File{name} => from_file(vm, reader, name),
        Command::String{string} => term::string::exec(vm, reader, string),
        Command::Line => term::line::exec(vm, reader),
        Command::Term => term::readline::exec(vm, reader),
    };

    let stack = vm.stack();
    if stack.len() > 0 {
        println!("Quitting with stack {:?}", vm.stack())
    };

    result
}
