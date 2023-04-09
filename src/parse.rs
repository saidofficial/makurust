use pulldown_cmark::{html, Options, Parser};
use std::fs::File;
use std::io::{self, prelude::*};
use std::error::Error;

pub fn parse_file(input_filename: &str) -> Result<String, Box<dyn Error>> {
    let mut input_file = File::open(input_filename)?;
    let mut input_contents = String::new();
    input_file.read_to_string(&mut input_contents)?;
    Ok(input_contents)
}
