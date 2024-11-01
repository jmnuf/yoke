use std::process::ExitCode;

fn run(program: String, args: Vec<String>) -> Result<(), String> {
    println!("Hello world!");
    println!("] {} {}", program, args);
    println!("Arguments count: {}", args.len());
    Ok(())
}

fn main() -> ExitCode {
    let args:Vec<String> = std::env::args().collect();
    let program:String = args.remove(0);
    match run(program, args) {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("ERROR: {}", err);
            ExitCode::FAILURE
        }
    }
}
