struct Person {
    name: String,
    age: u32,
}
// 構造体に関係の深い関数を紐付ければオブジェクト指向のプログラミングのクラスのような扱いができる
impl Person {
    // &selfはself: Selfのシンタックスシュガー
    fn say_name(&self) {
        println!("I am {}.", self.name);
    }

    fn say_age(&self) {
        println!("I am {} year(s) old.", self.age);
    }
}

fn main() {
    let p = Person {
        name: String::from("Taro"),
        age: 20,
    };

    p.say_name();
    p.say_age();
}
