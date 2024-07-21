use std::fs;
use std::io::Result;

use leekscript_parser::ast::*;

fn main() -> Result<()> {
    let file = "test_ai";

    // Read the content of the file into a string
    let file_content = fs::read(file)?;
    let file_content = String::from_utf8(file_content).unwrap();
    let span = Span::new_extra(&file_content, file);

    let result = File::parse(span);

    match result {
        Ok((_, file)) => {
            println!("{:#?}", file);
            println !("\n----------------------------\n{}", file);
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
    
    Ok(())
}