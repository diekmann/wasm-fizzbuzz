extern "C" {
    fn c_main();
}

fn main() {
    println!("Hello, world from rust!");
    unsafe { c_main(); }
}
