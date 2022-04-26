use crate::error::{Result, SQLRiteError};

use std::fmt;

#[derive(Debug, PartialEq)]
pub enum MetaCommand {
    Exit,
    Help,
    Open(String),
    Unknown,
}

impl fmt::Display for MetaCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MetaCommand::Exit => f.write_str(".exit"),
            MetaCommand::Help => f.write_str(".help"),
            MetaCommand::Open(_) => f.write_str(".open"),
            MetaCommand::Unknown => f.write_str("Unknown command"),
        }
    }
}

impl MetaCommand {
    pub fn new(command: String) -> Self {
        let args: Vec<&str> = command.split_whitespace().collect();
        let cmd = args[0].to_owned();
        match cmd.as_ref() {
            ".exit" => MetaCommand::Exit,
            ".help" => MetaCommand::Help,
            ".open" => MetaCommand::Open(command),
            _ => MetaCommand::Unknown,
        }
    }
}

pub fn handle_meta_command(command: MetaCommand) -> Result<String> {
    match command {
        MetaCommand::Exit => std::process::exit(0),
        MetaCommand::Help => Ok(format!(
            "{}{}{}{}{}",
            "Special commands:\n",
            ".help - Display this message\n",
            ".open <FILENAME> - Reopens a persistent database.\n",
            ".ast <QUERY> - Show the abstract syntax tree of QUERY.\n",
            ".exit - Quits this application"
        )),
        MetaCommand::Open(args) => Ok(format!("To be implemented: {}", args)),
        MetaCommand::Unknown => Err(SQLRiteError::UnknownCommand(format!(
            "Unknown command or invalid arguments. Enter '.help'"
        ))),
    }
}
