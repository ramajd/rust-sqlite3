extern crate clap;
mod repl;

use clap::{crate_version, Command};
use repl::{get_config, REPLHelper};

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() -> rustyline::Result<()> {
    env_logger::init();

    let _matches = Command::new("Rust-SQLite3")
        .version("0.0.1")
        .author("Reza Alizadeh Majd <r.a.majd@gmail.com>")
        .about("SQLite3 DB engine - re-implemented by Rust")
        .get_matches();

    let config = get_config();
    let helper = REPLHelper::new();

    let mut repl = Editor::with_config(config);
    repl.set_helper(Some(helper));

    if repl.load_history(".history.tmp").is_err() {
        println!("No previous history.");
    }

    let mut count = 1;
    loop {
        if count == 1 {
            // TODO: Get info about the application name and version dynamically
            println!(
                "{}{}{}{}",
                format!("Rust-SQLite3 - {}\n", crate_version!()),
                "Enter .exit to quit.\n",
                "Enter .help for usage hints.\n",
                "Connected to a transient in-memory database."
            );
        }
        let p = format!("rust-sqlite3 | {}> ", count);
        repl.helper_mut() // our helper
            .expect("No helper found")
            .colored_prompt = format!("\x1b[1;32m{}\x1b[0m", p);

        let readline = repl.readline(&p);
        match readline {
            Ok(command) => {
                repl.add_history_entry(command.as_str());
                // println!("Line: {}", command);
                if command.eq(".exit") {
                    break;
                } else {
                    println!(
                        "Error: unknown command or invalid arguments: '{}'. Enter '.help'",
                        &command
                    );
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
        count += 1;
    }
    repl.append_history(".history.tmp").unwrap();
    Ok(())
}
