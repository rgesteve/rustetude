fn main() {
    print!("Hello, world, calling a function\n");
    gen_seq_list();
    print!("\nDone!\n");
    std::process::exit(0);
}

fn gen_seq_list() {
    for i in 0..12 { println!("{}", i); }
}
/* 
use std::iter::range;

fn gen_seq_list() {
    let numbers: Vec<u32> = range(1, 11).collect();
    println!("{:?}", numbers); // Output: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
}
*/

#[test]
fn multiply_test() {
    assert_eq!(6, 2*3);
}