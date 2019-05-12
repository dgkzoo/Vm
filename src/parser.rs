///
/// ## Parser
///
/// ## author
/// dgkzoo
///

pub const CMD_PUSH: &str = "PUSH";
pub const CMD_POP: &str = "POP";
pub const CMD_ARITH: &str = "ARITH";


///
/// パーサ
///
pub struct Parser {
    org_line: String,
    cmd: String,
    arg1: String,
    arg2: String
}

impl Parser {
    pub fn new(line:String) -> Parser {
        Parser {
            org_line: line,
            cmd: "".to_string(),
            arg1: "".to_string(),
            arg2: "".to_string()
         }
    }

    pub fn exec(&mut self) {
        let line = self.get_valid_line().to_string();
        // self.cmd = self.get_command_type().to_string();

        let mut vals = line.split_whitespace().peekable();
        self.cmd = vals.next().unwrap().to_string();
        if vals.peek() != None { 
            self.arg1 = vals.next().unwrap().to_string();
        }
        if vals.peek() != None { 
            self.arg2 = vals.next().unwrap().to_string();
        }
    }

    ///
    /// 空白行、コメントの場合、ブランク文字列を返す
    ///
    pub fn get_valid_line(&self) -> String {
        let mut line = self.org_line.trim().to_string();

        if line.is_empty() {
            return "".to_string();
        }
        if line.starts_with("//") {
            return "".to_string();
        }

        if line.contains("//") {
            let comment_line = line;
            let sep: Vec<&str> = comment_line.split("//").collect();
            line = sep.get(0).unwrap().to_string();
        }

        return line.trim().to_string();
    }
    
    ///
    /// コマンドタイプを返す
    ///
    pub fn get_command_type(&self) -> &str {
        if self.org_line.starts_with("push") {
            return CMD_PUSH;
        }
        if self.org_line.starts_with("pop") {
            return CMD_POP;
        }

        return CMD_ARITH;
    }

    pub fn get_cmd(&self) -> &str{
        return &self.cmd;
    }

    pub fn get_arg1(&self) -> &str{
        return &self.arg1;
    }

    pub fn get_arg2(&self) -> &str {
        return &self.arg2;
    }
}
