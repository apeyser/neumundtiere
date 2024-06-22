use std::io;
use std::io::Write;
use std::ops::Drop;
use std::path::{Path,PathBuf};

use rustyline::error::ReadlineError;
use rustyline::{self, DefaultEditor};
use dirs::home_dir;

use super::*;
use rpn::Vm;
use reader::Reader;

pub struct Lines {
    #[cfg(feature = "with-file-history")]
    save: Box<Path>,
    editor: DefaultEditor, 
}

impl Lines {
    pub fn new() -> rustyline::Result<Self> {
        let mut editor = DefaultEditor::new()?;
        #[cfg(feature = "with-file-history")]
        {
            let save = match home_dir() {
                None => PathBuf::from("./rpn.txt"),
                Some(mut path_buf) => {
                    path_buf.push("rpn.txt");
                    path_buf
                }
            }.into_boxed_path();
            if let Err(err) = editor.load_history(&*save) {
                let path = save.display();
                println!("Unable to read save file {path}: {err}");
            };
            Ok(Self {save, editor})
        }
        #[cfg(not(feature = "with-file-history"))]
        Ok(Self {editor})
    }
}

impl Drop for Lines {
    fn drop(&mut self) {
        #[cfg(feature = "with-file-history")]
        {
            let Self {save, editor} = self;
            if let Err(err) = editor.save_history(&(*save)) {
                let save = save.display();
                println!("Unable write save file {save}: {err}")
            }
        }
    }
}


impl Iterator for Lines {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        io::stdout().flush().unwrap();
        let Self {editor, ..} = self; 
        match editor.readline(">> ") {
            Ok(line) => {
                if let Err(err) = editor.add_history_entry(line.as_str()) {
                    println!("Readline error: {err}")
                };
                Some(line)
            },
            Err(error) => {
                match error {
                    ReadlineError::Interrupted => println!("CTRL-C"),
                    ReadlineError::Eof => println!("CTRL-D"),
                    err => println!("Error: {:?}", err),
                };
                None
            }
        }
    }
}

pub fn exec(vm: &mut Vm, reader: &Reader) {
    match Lines::new() {
        Ok(lines) => super::exec(vm, reader, lines),
        Err(err) => println!("Readline error: {err}"),
    }
}
