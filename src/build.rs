
fn main () {
    if cfg!(target_os = "vita") {
        println!("cargo:rustc-toolchain=nightly");
        println!("cargo:rerun-if-env-changed=RUSTFLAGS");
    }
}