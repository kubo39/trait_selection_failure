use hoge::Hoge;
use fuga::Fuga;

fn main() {
    let hoge: Hoge = Hoge {};
    let fuga: Fuga = hoge.into();
}
