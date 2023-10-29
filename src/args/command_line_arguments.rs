pub trait CommandLineArguments<'a> {
    fn parse(args: &'a [String]) -> Result<Self, &'static str>
    where
        Self: std::marker::Sized;
}

pub fn has_flag(args: &[String], flag: &'static str) -> bool {
    args.iter().any(|arg| arg == flag)
}
