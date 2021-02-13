extern crate lzma;

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use lzma::reader::LzmaReader;

/// Xzips unifont .hex files for embedding in executable
fn main() -> std::io::Result<()> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir);

    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let data_dir = Path::new(&project_dir).join("data");

    let files = ["unifont-11.0.02.hex", "unifont_upper-11.0.02.hex"];

    for f in files.iter() {
        let handle = File::open(&data_dir.join(f))?;
        let mut comp = LzmaReader::new_compressor(handle, 6).unwrap();

        let mut content = Vec::new();
        comp.read_to_end(&mut content)?;

        let mut out = File::create(&out_path.join(f).with_extension("hex.xz"))?;
        out.write_all(&content)?;
    }

    Ok(())
}
