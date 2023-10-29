use clu::errors::Result;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        return Err("[ERROR] Requires atleast 1 argument.".into());
    }

    clu::handle_command(&args[1], &args[2..])
}
