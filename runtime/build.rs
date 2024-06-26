// NOTE: Adapted from riscv-rt

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let name = env::var("CARGO_PKG_NAME").unwrap();

    if !target.starts_with("riscv32i-") && !target.starts_with("riscv32imc-") {
        panic!("Unsupported target: {}", target);
    }

    let archive = format!("bin/{}.a", target);
    eprintln!("{}", archive);

    fs::copy(&archive, out_dir.join(format!("lib{}.a", name))).unwrap();
    println!("cargo:rerun-if-changed={}", archive);
    println!("cargo:rustc-link-lib=static={}", name);

    // Put the linker script somewhere the linker can find it
    fs::write(out_dir.join("link.x"), include_bytes!("link.x")).unwrap();
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed=link.x");
}
