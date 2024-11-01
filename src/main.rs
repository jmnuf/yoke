use std::process::ExitCode;
use std::path::PathBuf;
use std::io::{self, Write};

#[derive(Debug)]
enum SomeStr {
    Literal(&'static str),
    Dynamic(String)
}
impl std::fmt::Display for SomeStr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
	match self {
	    Self::Literal(s) => write!(f, "{}", s),
	    Self::Dynamic(s) => write!(f, "{}", s),
	}
    }
}
impl std::convert::From<&'static str> for SomeStr {
    fn from(value: &'static str) -> Self {
	Self::Literal(value)
    }
}
impl std::convert::From<String> for SomeStr {
    fn from(value: String) -> Self {
	Self::Dynamic(value)
    }
}

#[derive(Debug)]
struct RunFailure {
    error_message: SomeStr,
    display_usage: bool,
}
impl RunFailure {
    fn new<T: std::convert::Into<SomeStr>>(message: T, should_display_usage: bool) -> Self {
	Self {
	    error_message: message.into(),
	    display_usage: should_display_usage,
	}
    }
}

fn usage(program: &String) {
    println!("Usage: {} <FileA> [FileB [FileC [...]]]", program);
    println!("    -o <FILE>      --  Output concatenation to a file");
    println!("    -h             --  Display this help message");
}

fn run(program: &String, args: Vec<String>) -> Result<(), RunFailure> {
    if args.is_empty() {
	let failure = RunFailure::new("At least one file must be provided!", true);
	return Err(failure);
    }
    let mut output_path = None;
    let mut expecting_output = false;
    let mut paths = Vec::new();
    for arg in args.iter() {
	if expecting_output {
	    output_path = Some(PathBuf::from(arg));
	    expecting_output = false;
	    continue;
	}
	if arg == "-o" {
	    expecting_output = true;
	    continue;
	}
	if arg == "-h" || arg == "--help" || arg == "/?" {
	    usage(program);
	    return Ok(());
	}
	let path = PathBuf::from(arg);
	if ! path.is_file() {
	    let failure = RunFailure::new(format!("Non-file passed in: {}", path.display()), false);
	    return Err(failure);
	}
	paths.push(path);
    }
    if expecting_output {
	let failure = RunFailure::new("Was expecting an output path after `-o` but got nothing", false);
	return Err(failure);
    }

    let mut buffer = Vec::new();
    buffer.try_reserve(1024).expect("You need more RAM, sorry bucko");
    for p in paths {
	match std::fs::read(&p) {
	    Ok(mut data) => {
		buffer.append(&mut data);
	    },
	    Err(err) => {
		let failure = RunFailure::new(format!("Failed to read file `{}`: {}", p.display(), err), false);
		return Err(failure);
	    },
	};
    }

    let result = match output_path {
	Some(output_path) => {
	    let mut file = match std::fs::File::create(&output_path) {
		Err(err) => {
		    let failure = RunFailure::new(format!("Failed to open file `{}`: {}", output_path.display(), err), false);
		    return Err(failure);
		},
		Ok(f) => f,
	    };
	    
	    write_contents(&mut file, &buffer)
	},
	None => {
	    let stdout = io::stdout();
	    let mut handle = stdout.lock();
	    
	    write_contents(&mut handle, &buffer)
	}
    };

    match result {
	Err(err) => {
	    let failure = RunFailure::new(format!("Failed to write to output: {}", err), false);
	    return Err(failure)
	},
	Ok(_) => {},
    };

    Ok(())
}

fn write_contents(f:&mut impl Write, contents: &[u8]) -> io::Result<()> {
    f.write_all(contents)
}

fn main() -> ExitCode {
    let mut args:Vec<String> = std::env::args().collect();
    let program:String = args.remove(0);
    match run(&program, args) {
        Ok(_) => ExitCode::SUCCESS,
        Err(failure) => {
	    eprintln!("[ERROR] {}", failure.error_message);
	    if failure.display_usage {
		usage(&program);
	    }
	    ExitCode::FAILURE
        }
    }
}
