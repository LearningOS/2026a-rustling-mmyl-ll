// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let res = 42;
    let option = Some(12);
    let res = option.map_or(res, |x| res + x);
    println!("{}", res);
}
