use clap::Parser;
use clap_stdin::FileOrStdin;
use std::io::Read;

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

    /// The number of characters in each input file is written to the standard output.  If the current locale does not support multibyte characters, this is equivalent to the -c option.  This will cancel out any prior usage of the -c option.
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

    let content = match args.file.source {
        clap_stdin::Source::Arg(ref name) => std::fs::read_to_string(name).unwrap(),
        clap_stdin::Source::Stdin => {
            let mut buffer = String::new();
            std::io::stdin().read_to_string(&mut buffer).unwrap();
            buffer
        }
    };

    let name = match args.file.source {
        clap_stdin::Source::Arg(name) => name,
        _ => String::from(""),
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
