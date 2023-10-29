pub mod args;
pub mod errors;
pub mod grep;
use errors::Result;
pub mod utils;

pub fn handle_command(command: &str, args: &[String]) -> Result<()> {
    match command {
        "grep" => grep::run(args),
        arg => Err(format!("{arg} is not a valid command!")),
    }
}
