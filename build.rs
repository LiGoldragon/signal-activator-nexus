use ethos_zero::{Actualizing as _, File, Generating as _, Potential};

fn main() {
    println!("cargo:rerun-if-changed=ethos/signal.ethos");
    let source = std::fs::read_to_string("ethos/signal.ethos").expect("authored contract");
    let file = Potential::<File>::from(source).actualize().expect("valid Ethos");
    let generated = file.generate().expect("generated Signal vocabulary");
    let directory = std::path::PathBuf::from(std::env::var_os("OUT_DIR").expect("build directory"));
    std::fs::write(directory.join("signal.rs"), generated).expect("generated Rust");
}
