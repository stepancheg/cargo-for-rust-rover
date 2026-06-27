use std::fmt::Display;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use jiff::Timestamp;

pub(crate) struct Logger {
    file: Option<File>,
}

impl Logger {
    pub(crate) fn open(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref();
        let file = match OpenOptions::new().create(true).append(true).open(path) {
            Ok(file) => Some(file),
            Err(err) => {
                eprintln!("warning: failed to open log file {}: {err}", path.display());
                None
            }
        };
        Logger { file }
    }

    pub(crate) fn log(&mut self, message: impl Display) {
        let Some(file) = &mut self.file else {
            return;
        };
        let now = Timestamp::now();
        if let Err(err) = writeln!(file, "{now:.3} {message}") {
            eprintln!("warning: failed to write log line: {err}");
        }
    }
}
