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

    #[cfg(feature = "bindgen")]
    generate_bindings();
}

#[cfg(feature = "bindgen")]
fn generate_bindings() {
    bindgen::Builder::default()
        .header("miniaudio/extras/miniaudio_split/miniaudio.h")
        .raw_line("#![allow(non_upper_case_globals)]")
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(non_snake_case)]")
        .impl_debug(true)
        .prepend_enum_name(false)
        .rustfmt_bindings(true)
        .size_t_is_usize(true)
        .generate()
        .expect("unable to generate bindings")
        .write_to_file("src/lib.rs")
        .expect("couldn't write bindings");
}
