fn main() {
    cc::Build::new().cpp(true)
        .file("src/bork.cxx").compile("libbork.a");

    // Removed this in favor of automating building it from the buildscript,
    // using the cc crate which makes this way easy.
    // You can manually do this by doing
    //   g++ src/bork.c -c -o src/bork.o
    //   ar rcs src/libbork.a src/bork.o
    // and using the two lines below here.
    // println!("cargo:rustc-link-search=./src/");
    // println!("cargo:rustc-flags=-lstdc++");
}
