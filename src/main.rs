fn main() {
    print!("Hello, world, using exit code!\n");
    std::process::exit(0);
}

#[test]
fn multiply_test() {
    assert_eq!(6, 2*3);
}