use clap::Parser;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

// まずderive記法とは
//https://doc.rust-jp.rs/book-ja/ch19-06-macros.html?highlight=derive#%E3%83%9E%E3%82%AF%E3%83%AD
#[derive(Parser, Debug)]
#[clap(
    name = "My RPN program",
    version = "1.0.0",
    author = "naoya",
    about = "hoge"
)]
struct Opts {
    #[clap(short, long)]
    verbose: bool,

    #[clap(name = "FILE")]
    // Optionはデータがあることと無いことを表す構造型
    formula_file: Option<String>,
}

struct RpnCalculator(bool);

impl RpnCalculator {
    // Selfは呼び出し元オブジェクトの型。この場合はRpnCalculator
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    // &strがわからん.まず&がわからん
    pub fn eval(&self, formula: &str) -> i32 {
        // mutとVecがわからん
        // 確かmutは可変の変数
        // collectはイテレータをコレクションに変換する。変換先のコレクションを::<T>のように指定できる
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        // &mutもわからん
        self.eval_inner(&mut tokens)
    }

    fn eval_inner(&self, tokens: &mut Vec<&str>) -> i32 {
        0
    }
}

fn main() {
    let opts = Opts::parse();

    // pathってどこでも定義していないのになぜ使えるの？
    // formula_fileがある場合の値ってことか
    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        run(reader, opts.verbose)
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose)
    }
}

// lines()関数はBufReadというトレイトを実装していれば利用できる
// traitがあまり分からないのでtrait.rsに実装
fn run<R: BufRead>(reader: R, verbose: bool) {
    let calc = RpnCalculator::new(verbose);

    for line in reader.lines() {
        let line = line.unwrap();
        let answer = calc.eval(&line);
        println!("{}", answer)
    }
}
