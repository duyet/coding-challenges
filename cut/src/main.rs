use clap::Parser;
use std::path::PathBuf;

/// word, line, character, and byte count
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The list specifies fields, separated in the input by the field delimiter character (see the -d option).  Output fields are separated by a single occurrence of the field delimiter character.
    #[arg(short = 'f')]
    f: String,

    // Use delim as the field delimiter character instead of the tab character.
    #[arg(short = 'd')]
    d: Option<String>,

    file: PathBuf,
}

fn main() {
    let args = Args::parse();

    let f_list = args
        .f
        .split(&[' ', ','])
        .filter(|&r| r != "")
        .collect::<Vec<&str>>();

    let delim = args.d.unwrap_or("\t".to_string());

    // Read the file content
    match std::fs::read_to_string(args.file.clone()) {
        Ok(content) => {
            println!("{}", cut(&content, f_list, &delim));
        }
        _ => println!("Error reading file: {:?}", args.file),
    }
}

fn cut(content: &str, f_list: Vec<&str>, delim: &str) -> String {
    let mut result = String::new();

    for line in content.lines() {
        let fields = line.split(delim).collect::<Vec<&str>>();

        f_list.iter().for_each(|f| {
            let index = f.parse::<usize>().unwrap();
            if index <= fields.len() {
                result.push_str(fields[index - 1]);

                if f != f_list.last().unwrap() {
                    result.push_str(delim);
                }
            }
        });

        result.push_str("\n");
    }

    result
}
