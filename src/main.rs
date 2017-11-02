extern crate itertools;
use std::io::{stdin, Read};
use itertools::Itertools;
use std::str::from_utf8;

fn main() {

    let stdin = stdin();
    let rle: String = stdin
        .lock()
        .bytes()
        .map(|b| b.unwrap())
        .group_by(|&k| from_utf8(&[k]).unwrap().to_owned())
        .into_iter()
        .map(|(k, grp)| format!("{}{}", k, grp.count()))
        .join("");

    println!("{}", rle);
}
