use clap::Parser;
use std::io::Read;
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};

// In case wc is called with multiple stdin arguments, we want to error out.
static STDIN_HAS_BEEN_USED: AtomicBool = AtomicBool::new(false);

#[derive(Debug, thiserror::Error)]
pub enum StdinError {
    #[error("stdin argument used more than once")]
    StdInRepeatedUse,
    #[error(transparent)]
    StdIn(#[from] std::io::Error),
    #[error("unable to parse from_str: {0}")]
    FromStr(String),
}

#[derive(Debug, Clone)]
enum FileOrStdin {
    Arg(String),
    Stdin,
}

impl FromStr for FileOrStdin {
    type Err = StdinError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => {
                if STDIN_HAS_BEEN_USED.load(Ordering::Acquire) {
                    return Err(StdinError::StdInRepeatedUse);
                }
                STDIN_HAS_BEEN_USED.store(true, Ordering::SeqCst);
                Ok(Self::Stdin)
            }
            arg => Ok(Self::Arg(arg.to_owned())),
        }
    }
}

impl FileOrStdin {
    fn contents(&self) -> Result<String, StdinError> {
        match self {
            Self::Arg(name) => std::fs::read_to_string(name).map_err(StdinError::StdIn),
            Self::Stdin => {
                let mut buffer = String::new();
                std::io::stdin().read_to_string(&mut buffer)?;
                Ok(buffer)
            }
        }
    }

    fn name(&self) -> String {
        match self {
            Self::Arg(name) => name.clone(),
            Self::Stdin => String::from(""),
        }
    }
}

/// word, line, character, and byte count
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The number of bytes in each input file is written to the standard output.
    #[arg(short = 'c')]
    c: bool,

    /// The number of lines in each input file is written to the standard output.
    #[arg(short = 'l')]
    l: bool,

    /// The number of characters in each input file is written to the standard output. If the current locale does not support multibyte characters, this is equivalent to the -c option.  This will cancel out any prior usage of the -c option.
    #[arg(short = 'm')]
    m: bool,

    /// The number of words in each input file is written to the standard output.
    #[arg(short = 'w')]
    w: bool,

    /// File path
    #[clap(default_value = "-")]
    file: FileOrStdin,
}

fn main() {
    let args = Args::parse();

    let name = args.file.name();

    let content = match args.file.contents() {
        Ok(content) => content,
        Err(e) => {
            println!("wc: {:?}: {}", args.file, e);
            std::process::exit(1);
        }
    };

    let array = [
        (args.l, get_l(&content)),
        (args.w, get_w(&content)),
        (args.c, get_c(&content)),
        (args.m, get_m(&content)),
    ];

    // no options are provided, which is the equivalent to the -c, -l and -w options
    if !array.iter().any(|&x| x.0) {
        println!(
            "{:>8}{:>8}{:>8} {}",
            get_l(&content),
            get_w(&content),
            get_c(&content),
            name
        );
    } else {
        array.iter().for_each(|&x| {
            if x.0 {
                print!("{:>8}", x.1);
            }
        });

        println!(" {}", name);
    }
}

// -l: get lines
fn get_l(content: &str) -> usize {
    content.lines().count()
}

// -w: get words
fn get_w(content: &str) -> usize {
    content.split_whitespace().count()
}

// -m: get characters
fn get_m(content: &str) -> usize {
    content.chars().count()
}

// -c: get bytes
fn get_c(content: &str) -> usize {
    content.len()
}
