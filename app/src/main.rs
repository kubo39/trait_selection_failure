use hoge::Hoge;
use fuga::Fuga;

fn main() {
    let hoge: Hoge = Hoge {};
    let _fuga: Fuga = hoge.into();
}
