fn main() {
    let mut build = cc::Build::new();
    build.include("src/c");
    build.define("SOKOL_D3D11", None);
    build.define("SOKOL_FONTSTASH_IMPL", None);
    build.define("FONTSTASH_IMPLEMENTATION", None);
    build.file("src/c/sokol_fontstash.c");
    build.compile("fons");
    bindgen::Builder::default()
        .header("src/c/fontstash.h")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("src/fons.rs")
        .expect("Couldn't write bindings!");
}
