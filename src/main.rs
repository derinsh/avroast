use std::io;
use std::io::prelude;
use std::fs::File;
use std::env;
use std::collections::HashMap;

mod bit;
mod cli;
mod sys;
mod av;

use av::Operation;

struct Roast {
    map: HashMap<File, Operation>,
}

// TODO: basic parsing and reading pcm from wav file

fn main() {

    let command : Vec<String> = env::args().collect();
    let parsed = parse(command);

    // TODO
    if parsed.len() > 0 {
        cli::main(command)
    }
    ()
}

fn parse(cmd : Vec<String>) -> HashMap<String, String> {
    let parsed : HashMap<String, String> = HashMap::new();
    parsed
}
