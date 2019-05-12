///
/// ## VM・メイン
///
/// ## author
/// dgkzoo
///
use std::env;

extern crate vm;
use vm::vmtransrator::VmTransrator;

fn main() {
    // 引数チェック
    if env::args().len() != 2 {
        println!("引数が不正です。*.vmファイルを１つ指定してください");
        return;
    }

    // ターゲットファイルの取得
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];

    // vmファイルからasmファイルを生成
    let mut vmt = VmTransrator::new();
    vmt.translate(filepath.to_string());
}
