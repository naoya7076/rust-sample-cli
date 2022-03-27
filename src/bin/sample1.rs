use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args)
    // {:?}と{}は何が違うの？
    // この辺りが参考になりそう
    // http://doc.rust-jp.rs/rust-by-example-ja/hello/print/print_display.html
}
