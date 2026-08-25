mod grammar;// default namespace for the parser is the grammar's name
pub mod program;
pub mod unit_check;
pub mod equiv_convert;
pub mod unit_convert;
pub mod soundness;
pub mod utils;
pub mod monitor_setup;
pub mod monitor;
mod errors;
extern crate hime_redist;
use std::fs;
use colored::Colorize;
use dotenv::dotenv;
// use monitor::instrumentation::Instrumentation;
use crate::program::Program;


#[tokio::main]
async fn main() {
    dotenv().ok();
    // let base_url = std::env::var("BASE_URL").expect("BASE_URL not defined in .env");
    // let token = std::env::var("TOKEN").expect("TOKEN not defined in .env");

    // let instrumentation = match Instrumentation::new(&base_url, &token) {
    //     Ok(instrumentation) => instrumentation,
    //     Err(err) => return error_print(format!("{}", err)),
    // };

    let program_str = match fs::read_to_string("program.txt") {
        Ok(program_str) => program_str,
        Err(err) => return error_print(format!("{}",err))
    };
   
    let mut program = match Program::new(program_str.as_str()) {
        Ok(program) => program,
        Err(err) => return error_print(format!("{}",err))
    };

    if let Err(err) = program.unit_convert() {
        return error_print(format!("{}",err));
    };

    if let Err(err) = program.unit_check() {
        return error_print(format!("{}",err));
    }

    if let Err(err) = program.equiv_convert() {
        return error_print(format!("{}",err));
    };
    
    if let Err(err) = program.soundness_check() {
        return error_print(format!("{}",err));
    }

    if let Err(err) = program.compile_properties() {
        return error_print(format!("{}",err));
    }

    println!("{:#?}", program.environment);
    
    // if let Err(err) = program.monitor(instrumentation, 1_000, false).await {
    //     return error_print(format!("{}",err));
    // }
}

fn error_print(err: String) {
    println!("{}{}",String::from("[Error]: ").bright_red().bold(), err)
}
