///
/// ## Parser
///
/// ## author
/// dgkzoo
///

///
/// パーサ
///
pub struct Parser {}

impl Parser {
    pub fn new() -> Parser {
        Parser {}
    }

    ///
    /// 空白行、コメントの場合、ブランク文字列を返す
    ///
    pub fn get_valid_line(&self, line: &mut String) -> String {
        let mut line = line.trim().to_string();

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
}
