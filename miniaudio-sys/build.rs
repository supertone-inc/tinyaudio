fn main() {
    let mut cc_builder = cc::Build::new();

    cc_builder
        .include("miniaudio/extras/miniaudio_split")
        .file("miniaudio/extras/miniaudio_split/miniaudio.c")
        .cpp(false)
        .flag_if_supported("-Wno-deprecated-declarations")
        .flag_if_supported("-Wno-unused-but-set-variable")
        .flag_if_supported("-Wno-unused-function")
        .compile("miniaudio");

    bindgen::Builder::default()
        .header("miniaudio/extras/miniaudio_split/miniaudio.h")
        .impl_debug(true)
        .prepend_enum_name(false)
        .rustfmt_bindings(true)
        .size_t_is_usize(true)
        .generate()
        .expect("unable to generate bindings")
        .write_to_file("src/bindings.rs")
        .expect("couldn't write bindings");

    println!("cargo:rerun-if-changed=miniaudio/extras/miniaudio_split/miniaudio.h");
    println!("cargo:rerun-if-changed=miniaudio/extras/miniaudio_split/miniaudio.c");
    println!("cargo:rerun-if-env-changed=CC");
}
