use std::{env, fs, process};

fn main() {
    let args = env::args().collect::<Vec<String>>();

    dbg!(&args);

    let Config { query, file_path } = Config::build(&args).unwrap_or_else(|message| {
        println!("Problem when parsing args: {message}.");
        process::exit(1);
    });

    let file_contents =
        fs::read_to_string(file_path).expect(&format!("Expected to read {file_path}"));

    println!("{file_contents}");
}

struct Config<'a> {
    query: &'a String,
    file_path: &'a String,
}

impl<'a> Config<'a> {
    fn build(args: &'a [String]) -> Result<Self, &'static str> {
        let Some(query) = args.get(1) else {
            return Err("Missing query argument!");
        };

        let Some(file_path) = args.get(2) else {
            return Err("Missing file_path argument!");
        };

        Ok(Config { query, file_path })
    }
}
