pub trait CommandLineArguments<'a> {
    fn parse(args: &'a [String]) -> Result<Self, &'static str>
    where
        Self: std::marker::Sized;
}
