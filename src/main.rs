use clap::Parser;

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
    formula_file: Option<String>,
}

fn main() {
    let opts = Opts::parse();
    match opts.formula_file {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified."),
    }
    println!("Is verbosity specified?:{}", opts.verbose)
}
