use clu::errors::Result;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 {
        return Err("[ERROR] Requires atleast 1 argument.".into());
    }

    clu::handle_command(&args)
}
