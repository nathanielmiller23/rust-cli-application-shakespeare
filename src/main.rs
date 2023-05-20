#![allow(unused)]
//This piece of code was added to prevent warnings about unused code. Naturally I will have unused code if I am not yet finished with the project

use std::io;

use std::error::Error;
//This is a module from the standard library that allows for us to use the Error trait.

use anyhow::{Context, Result};
//Anyhow is a library for error handling in Rust. This allows you to attach information to your errors instead of simply looking at broken code. The Context trait extends the Result type with additional methods for attaching context information to error values. It allows you to provide more context or additional information when an error occurs, making it easier to understand the error's origin or provide meaningful error messages.

use clap::Parser;
//This is how to import a dependency in Rust
//Similar to other languages, but you use :: to specify the particular dependency you want to import
//This is a macro that imports the clap crate, which contains the Parser trait 

#[derive(Parser)]
//By using derive, you don't have to manually write the implementation for the trait (Parser in this case), as the Rust compiler generates the required code behind the scenes.


struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}
//This is a struct that represents the command line arguments. It has a field for the pattern and a field for the path (Update if searching for a specific file)


fn main() -> Result<()> {

    println!("Select a Sonnet");
    println!("");
    println!("Sonnet18.txt");
    println!("Sonnet102.txt");
    println!("Sonnet105.txt");
    println!("Sonnet112.txt");
    println!("Sonnet145.txt");
    println!("Sonnet147.txt");
    println!("");
    let mut path = String::new();
  
    println!("Enter the file name:");
    io::stdin()
        .read_line(&mut path)
        .expect("Failed to read input");
    let path = path.trim();
  
    //let path = "Sonnet18.txt";
    //Here, we define the path for the file we want to read
  
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("could not read file `{}`", path))?;
    //The content is set equal to the contents of the file
    println!("");
    println!("File content: {}", content);
  //This prints the contents of the file
  
    Ok(())
  //This is a return statement that indicates that the function has completed successfully.   It is important to return a value of type Result<()>, which is the return type of the main function.
}

