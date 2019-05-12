///
/// ## VM Tranrator
///
/// ## author
/// dgkzoo
///
use std::fs;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

use parser;
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
            let parser = Parser::new(line.unwrap());
            let line = parser.get_valid_line();
            if line.is_empty() {
                continue;
            }

            // ファイルに出力
            let out_asm = self.vm2asm(line);
            print!("out_asm = {}", out_asm);
            out_buf.write(out_asm.as_bytes()).unwrap();
        }
    }

    ///
    /// VMコマンドをasmに変換する
    /// 
    pub fn vm2asm(&mut self, line: String) -> String{
        let mut out_asm = "".to_string();

        let mut parser = Parser::new(line);
        parser.exec();
        let cmd_type = parser.get_command_type();
        let cmd = String::from(parser.get_cmd());
        let arg1 = String::from(parser.get_arg1());
        let arg2 = String::from(parser.get_arg2());

        // スタック系（push, pop）
        if cmd_type == parser::CMD_PUSH || cmd_type == parser::CMD_POP {
            out_asm = self.vm2asm_push_pop(cmd_type.to_string(), arg1, arg2);
        }

        // 計算系（addとか）
        if cmd_type == parser::CMD_ARITH {
            out_asm = self.vm2asm_arithmethic(cmd.to_string());
        }

        return out_asm;
    }

    ///
    /// pushコマンドをasmへ変換
    /// 
    pub fn vm2asm_push_pop(&mut self, command:String, segment:String, index:String) -> String {
        let mut out_asm = "".to_string();

        // constant値をpush
        if segment == "constant" {
            out_asm = out_asm + format!("@{}    // ***push constant {} ***\n", index, index).as_str();
            out_asm = out_asm + format!("D=A    // D=A(constant {})\n", index).as_str();
            out_asm = out_asm + format!("@SP    // Areg=0x00\n").as_str();
            out_asm = out_asm + format!("A=M    // A=M[SP]\n").as_str();
            out_asm = out_asm + format!("M=D    // push ({}) // M[SP]=D(constant {})\n", index, index).as_str();
            out_asm = out_asm + format!("@SP    // Areg=0x00\n").as_str();
            out_asm = out_asm + format!("M=M+1  // SP inc // M[SP]=M[SP]+1\n").as_str();
        }

        return out_asm;
    }

    ///
    /// pushコマンドをasmへ変換
    /// 
    pub fn vm2asm_arithmethic(&self, command:String) -> String {
        let mut out_asm = "".to_string();

        // add
        if command == "add" {
            out_asm = out_asm + format!("@SP   // ***add***\n").as_str();
			out_asm = out_asm + format!("M=M-1 // SP - 1\n").as_str();
			out_asm = out_asm + format!("A=M   // A=M[SP](SP Address)\n").as_str();
			out_asm = out_asm + format!("D=M   // D=M(val2 to D)\n").as_str();

			out_asm = out_asm + format!("@SP   // Areg=0\n").as_str();
			out_asm = out_asm + format!("M=M-1 // SP-1\n").as_str();
			out_asm = out_asm + format!("A=M   // A=M[SP](SP Address)\n").as_str();
			out_asm = out_asm + format!("M=D+M // add\n").as_str();

			out_asm = out_asm + format!("@SP   // Areg=0\n").as_str();
			out_asm = out_asm + format!("M=M+1 // SP + 1\n").as_str();

			return out_asm;
        }

        return out_asm;
    }
}
