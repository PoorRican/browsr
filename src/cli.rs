use std::process::exit;

use getopts::Occur;

use args::{Args, ArgsError};

use crate::mode::{Mode, DetailMode};

const PROGRAM_NAME: &'static str = "browsr";
const PROGRAM_DESC: &'static str = "Browse biological annotation data";

pub fn parse_args(input: &Vec<String>) -> Result<Box<dyn Mode>, ArgsError> {
    let mut args = Args::new(PROGRAM_NAME, PROGRAM_DESC);
    args.flag("h", "help", "Print the usage menu");
    args.option("f",
        "filename",
        "The name of the annotation file to open",
        "NAME",
        Occur::Optional,
        Some(String::from("annotation.xml")));

    args.parse(input)?;

    let help = args.value_of("help")?;
    if help {
        println!("{}", args.full_usage());
        exit(1);
    }
    if args.has_value("filename") {
        let filename = args.value_of("filename")?;
        Ok(Box::new(DetailMode::new(filename)))
    } else {
        panic!("I have nothing to do...");
    }
}
