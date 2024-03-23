use std::fs;

pub fn minigrap(query: &String, file_path: &String) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file!");

    println!("with text \n {contents}");
}

pub fn parse_config(args: &[String]) -> (&String, &String) {
    let query = &args[1];
    let filepath = &args[2];

    println!("Searching for string {query}");
    println!("In file: {filepath}");

    return (query, filepath);
}
