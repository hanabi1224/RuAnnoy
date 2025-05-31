use rustc_version::{version_meta, Channel};

fn main() {
    println!("cargo::rustc-check-cfg=cfg(beta)");
    println!("cargo::rustc-check-cfg=cfg(nightly)");
    match version_meta().unwrap().channel {
        Channel::Nightly => {
            println!("cargo:rustc-cfg=nightly");
        }
        Channel::Beta => {
            println!("cargo:rustc-cfg=beta");
        }
        _ => {}
    };
}
