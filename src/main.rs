#[link(name = "bork")]
extern "C" {
    fn bork();
}

fn main() {
    println!("Hello, world, from Rust.");
    unsafe { bork() };
}
