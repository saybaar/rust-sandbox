use std::io::Write;
use std::str::FromStr;

fn main() {
    let mut nums = Vec::new();
    for arg in std::env::args().skip(1) {
        nums.push(u64::from_str(&arg)
            .expect("hello, this is a parse error"));
    }
    if nums.len() == 0 {
        writeln!(std::io::stderr(), "Usage: hello CHAR...").unwrap();
    }
    for c in nums {
        println!("Found {}", c);
    }
    println!("Done");
}

#[test]
fn test_logic() {
    assert_eq!(true, !false);
}
