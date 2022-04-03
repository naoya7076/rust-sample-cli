use anyhow::{bail, ensure, Context, Result};

use clap::Parser;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::PathBuf;

struct RpnCalculator(bool);

impl RpnCalculator {
    // Selfは呼び出し元オブジェクトの型。この場合はRpnCalculator
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    // &strがわからん.まず&がわからん
    pub fn eval(&self, formula: &str) -> Result<i32> {
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    fn eval_inner(&self, tokens: &mut Vec<&str>) -> Result<i32> {
        let mut stack = Vec::new();
        let mut pos = 0;

        while let Some(token) = tokens.pop() {
            pos += 1;
            if let Ok(x) = token.parse::<i32>() {
                stack.push(x)
            } else {
                let y = stack.pop().context(format!("invalid syntax at {}", pos))?;
                let x = stack.pop().context(format!("invalid syntax at {}", pos))?;
                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => bail!("invalid token at {}", pos),
                };
                stack.push(res)
            }

            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }

        ensure!(stack.len() == 1, "invalid syntax");

        Ok(stack[0])
    }
}

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
    formula_file: Option<PathBuf>,
}

// main関数ってResult返せれるのか。Result<()>の「()」は何だ
fn main() -> Result<()> {
    let opts = Opts::parse();

    // pathってどこでも定義していないのになぜ使えるの？
    // formula_fileがある場合の値ってことか
    if let Some(path) = opts.formula_file {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        run(reader, opts.verbose)
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose)
    }
}

// lines()関数はBufReadというトレイトを実装していれば利用できる
// traitがあまり分からないのでtrait.rsで学習
// Result<()>の()はなんだろうか
fn run<R: BufRead>(reader: R, verbose: bool) -> Result<()> {
    let calc = RpnCalculator::new(verbose);

    for line in reader.lines() {
        let line = line?;
        match calc.eval(&line) {
            Ok(answer) => println!("{}", answer),
            Err(e) => println!("{:#?}", e),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_ok() {
        let calc = RpnCalculator::new(false);
        assert_eq!(calc.eval("5").unwrap(), 5);
        assert_eq!(calc.eval("50").unwrap(), 50);
        assert_eq!(calc.eval("-50").unwrap(), -50);

        assert_eq!(calc.eval("2 3 +").unwrap(), 5);
        assert_eq!(calc.eval("2 3 *").unwrap(), 6);
        assert_eq!(calc.eval("2 3 -").unwrap(), -1);
        assert_eq!(calc.eval("2 3 /").unwrap(), 0);
        assert_eq!(calc.eval("2 3 %").unwrap(), 2);
    }

    #[test]
    fn test_ng() {
        let calc = RpnCalculator::new(false);
        assert!(calc.eval("").is_err());
        assert!(calc.eval("1 1 1 +").is_err());
        assert!(calc.eval("+ 1 1").is_err());
    }
}
