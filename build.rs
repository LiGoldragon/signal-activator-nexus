use ethos_zero::{Actualizing as _, File, Generating as _, Potential};

fn main() {
    println!("cargo:rerun-if-changed=ethos/signal.ethos");
    let source = std::fs::read_to_string("ethos/signal.ethos").expect("authored contract");
    let file = Potential::<File>::from(source)
        .actualize()
        .expect("valid Ethos");
    let generated = file.generate().expect("generated Signal vocabulary");
    let directory = std::path::PathBuf::from(std::env::var_os("OUT_DIR").expect("build directory"));
    let generated_path = directory.join("signal.rs");
    std::fs::write(&generated_path, generated).expect("generated Rust");
    let entry = format!(
        "#[path = {:?}] pub mod generated;\npub use generated::*;\n",
        generated_path
    );
    std::fs::write(directory.join("entry.rs"), entry).expect("module entry");
}
