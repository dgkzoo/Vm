///
/// ## VM Tranrator
///
/// ## author
/// dgkzoo
///
use std::fs;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

use parser::Parser;

pub struct VmTransrator {
}

impl VmTransrator {
    pub fn new() -> VmTransrator {
        VmTransrator {}
    }
    
    ///
    /// アセンブリへの変換
    /// 
    pub fn translate(&mut self, filepath: String) {
        println!("filepath = {}", filepath);

        let infile = fs::File::open(filepath.to_string()).unwrap();

        // 入出力のパス
        let inpath = Path::new(&filepath);
        let mut outfilepath = String::from(
            inpath
                .with_file_name(inpath.file_stem().unwrap())
                .to_str()
                .unwrap(),
        );
        outfilepath = outfilepath + ".asm";
        let mut out_buf = BufWriter::new(fs::File::create(outfilepath).unwrap());

        let reader = BufReader::new(infile);
        for line in reader.lines() {

            // コメント、空白行などを除去
            let parser = Parser::new();
            let line = parser.get_valid_line(&mut line.unwrap());
            if line.is_empty() {
                continue;
            }

            println!("line = {}", line);

            // ファイルに出力
            print!("{}", line);
            out_buf.write(line.as_bytes()).unwrap();
        }
    }
}
