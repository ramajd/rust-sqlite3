extern crate clap;
mod error;
mod meta_command;
mod repl;
mod sql;

use clap::{crate_authors, crate_description, crate_version, Command};
use meta_command::handle_meta_command;
use repl::{get_command_type, get_config, CommandType, REPLHelper};
use sql::process_command;

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() -> rustyline::Result<()> {
    env_logger::init();

    let _matches = Command::new("Rust-SQLite3")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .get_matches();

    let config = get_config();
    let helper = REPLHelper::new();

    let mut repl = Editor::with_config(config);
    repl.set_helper(Some(helper));

    if repl.load_history(".history.tmp").is_err() {
        println!("No previous history.");
    }

    loop {
        let p = format!("rust-sql> ");
        repl.helper_mut() // our helper
            .expect("No helper found")
            .colored_prompt = format!("\x1b[1;32m{}\x1b[0m", p);

        let readline = repl.readline(&p);
        match readline {
            Ok(command) => {
                repl.add_history_entry(command.as_str());

                match get_command_type(&command.trim().to_owned()) {
                    CommandType::SQLCommand(_cmd) => {
                        let _ = match process_command(&command) {
                            Ok(response) => println!("{}", response),
                            Err(err) => println!("An error occurred: {}", err),
                        };
                    }
                    CommandType::MetaCommand(cmd) => {
                        let _ = match handle_meta_command(cmd) {
                            Ok(response) => println!("{}", response),
                            Err(err) => println!("An error occurred: {}", err),
                        };
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    repl.append_history(".history.tmp").unwrap();
    Ok(())
}
