pub mod args;
pub mod errors;
pub mod grep;
use errors::Result;
pub mod utils;

pub fn handle_command(args: &[String]) -> Result<()> {
    let command = &args[1].as_str();
    let run_args = &args[2..]; // strip path and command

    match *command {
        "grep" => grep::run(run_args),
        arg => Err(format!("{arg} is not a valid command!")),
    }
}
