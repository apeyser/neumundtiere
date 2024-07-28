use std::io;
use std::io::Write;
use std::ops::Drop;
use std::path::{Path, PathBuf};
use std::env::{var, current_dir};

use rustyline::error::ReadlineError;
use rustyline::{self, DefaultEditor};
use dirs::home_dir;

use super::*;
use crate::reader::Reader;

pub struct Lines {
    #[cfg(feature = "with-file-history")]
    save: Box<Path>,
    editor: DefaultEditor, 
}

impl Lines {
    pub fn new() -> rustyline::Result<Self> {
        Self::build(DefaultEditor::new()?)
    }

    #[cfg(feature = "with-file-history")]
    fn build(mut editor: rustyline::DefaultEditor) -> rustyline::Result<Self> {
        const HISTFILE: Option<&'static str> = option_env!("DEUTEROSTOME_HISTORY");
        const HISTFILE_DIR: Option<&'static str> = option_env!("DEUTEROSTOME_HISTORY_DIR");
        
        let histfile = var("DEUTEROSTOME_HISTORY")
            .unwrap_or_else(|_| HISTFILE.unwrap_or(".deuterostome-history").to_string());
        let histfile_dir = var("DEUTEROSTOME_HISTORY_DIR")
            .unwrap_or_else(|_| HISTFILE_DIR.unwrap_or("~").to_string());
        
        let save = {
            let mut path_buf = if histfile_dir != "~" {
                PathBuf::from(histfile_dir)
            } else {
                match home_dir() {
                    None => current_dir()?,
                    Some(path_buf) => path_buf,
                }
            };
            path_buf.push(histfile);
            path_buf
        }.into_boxed_path();

        let path = save.display();
        eprintln!("Using {path} for history");
        if let Err(err) = editor.load_history(&*save) {
            eprintln!("Unable to read save file {path}: {err}");
        };
        Ok(Self {save, editor})
    }
    
    #[cfg(not(feature = "with-file-history"))]
    fn build(mut editor: rustyline::DefaultEditor) -> rustyline::Result<Self> {
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
                eprintln!("Unable write save file {save}: {err}")
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
                    eprintln!("Readline error: {err}")
                };
                Some(line)
            },
            Err(error) => {
                match error {
                    ReadlineError::Interrupted => eprintln!("CTRL-C"),
                    ReadlineError::Eof => eprintln!("CTRL-D"),
                    err => eprintln!("Error: {:?}", err),
                };
                None
            }
        }
    }
}

pub fn exec(reader: &mut Reader) -> MainResult {
    match Lines::new() {
        Ok(lines) => super::exec(reader, lines),
        Err(err) => {
            eprintln!("Readline error: {err}");
            Err(Box::new(err))
        }
    }
}
