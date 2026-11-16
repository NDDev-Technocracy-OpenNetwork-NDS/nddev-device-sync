#[path = "src/fingerprint.rs"]
mod fingerprint;

fn main() -> std::io::Result<()> {
    let manifest = std::path::PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let root = manifest.parent().unwrap();
    for path in fingerprint::inputs(root)? {
        println!("cargo:rerun-if-changed={}", root.join(path).display());
    }
    println!(
        "cargo:rustc-env=NDS_SOURCE_INPUTS={}",
        fingerprint::calculate(root)?
    );
    Ok(())
}
