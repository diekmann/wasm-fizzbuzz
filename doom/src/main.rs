extern "C" {
    fn D_DoomMain() -> !;
}

fn main() {
    println!("Hello, world from rust!");

    // TODO: set global variables
    // myargc=2 and myargv={"-2"}

    unsafe { D_DoomMain() };
}
