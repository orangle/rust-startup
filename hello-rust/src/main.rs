use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

mod value_tour;

fn main() {
    println!("Hello, world!");

    let mut arr:Vec<i32> = vec![2, 3, 1];
    arr.push(4);

    let stdout = stdout();
    let out = b"Hello fellow Rustaceans!";
    let width = 24;

    let mut writer = BufWriter::new(stdout.lock());
    say(out, width, &mut writer).unwrap();
}
