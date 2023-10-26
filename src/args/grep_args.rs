use super::command_line_arguments::CommandLineArguments;

#[derive(Debug, PartialEq)]
pub struct GrepArguments<'a> {
    pub filename: &'a str,
    pub patterns: Vec<String>,
    pub ignore_case: bool,
}

impl<'a> CommandLineArguments<'a> for GrepArguments<'a> {
    fn parse(args: &'a [String]) -> Result<Self, &'static str> {
        let filename = args
            .get(0)
            .ok_or("Missing arguments: filepath & patterns")?;

        let patterns: Vec<String> = args
            .get(1..)
            .ok_or("Missing arguments: patterns")?
            .iter()
            .filter(|arg| !arg.starts_with("--"))
            .cloned()
            .collect();

        let ignore_case = args.contains(&("--ignore-case".into()));

        Ok(GrepArguments {
            filename,
            patterns,
            ignore_case,
        })
    }
}

#[test]
fn test_parsing() {
    let args: Vec<String> = [
        String::new(),
        String::from("t.txt"),
        String::from("pattern"),
        String::from("--ignore-case"),
    ]
    .into_iter()
    .skip(1)
    .collect();

    assert_eq!(
        GrepArguments::parse(&args).unwrap(),
        GrepArguments {
            filename: "t.txt",
            patterns: vec![String::from("pattern")],
            ignore_case: true
        }
    )
}
