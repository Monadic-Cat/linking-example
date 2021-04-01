fn main() {
    cc::Build::new().cpp(true)
        .file("src/bork.cxx").compile("libbork.a");

    // Removed this in favor of automating building it from the buildscript,
    // using the cc crate which makes this way easy.
    // println!("cargo:rustc-link-search=./src/");
    // println!("cargo:rustc-flags=-lstdc++");
}
