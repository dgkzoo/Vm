///
/// ## VM Tranrator
///
/// ## author
/// dgkzoo
///
use std::fs;
use std::io::{BufRead, BufReader, BufWriter, Write};

use parser::Parser;

pub struct VmTransrator {
}

impl VmTransrator {
    pub fn new() -> VmTransrator {
        VmTransrator {}
    }
    
    pub fn translate(&mut self, filepath: String) {
        println!("filepath = {}", filepath);

        let infile = fs::File::open(filepath.to_string()).unwrap();
        let reader = BufReader::new(infile);
        for line in reader.lines() {
            // コメント、空白行などを除去
            let parser = Parser::new();
            let line = parser.get_valid_line(&mut line.unwrap());
            if line.is_empty() {
                continue;
            }

            println!("line = {}", line);
        }
    }
}
