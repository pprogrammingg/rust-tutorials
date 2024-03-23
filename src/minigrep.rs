use std::fs;
pub struct Config {
    query: String,
    file_path: String
}

pub fn minigrep(config: Config) {
    let contents = fs::read_to_string(config.file_path).expect("Should have been able to read the file!");

    println!("with text \n {contents}");
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("Not enough arguments are provided in the command line!");
        }
        Config {
            query: args[1].clone(),
            file_path: args[2].clone()
        }
    }
}
