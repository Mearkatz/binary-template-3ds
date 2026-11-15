use image::{GenericImageView, ImageFormat, imageops::FilterType};
use std::{fs, path::PathBuf, process};

const SIZE: u32 = 48;
const OUT_NAME: &str = "icon.png";

fn fail(msg: String) -> ! {
    eprintln!("error: {msg}");
    process::exit(1);
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    // After processing, the icon is always icon.png. If it's missing, Cargo reruns
    // this script each build, so a newly added icon gets picked up.
    println!("cargo:rerun-if-changed={OUT_NAME}");

    let dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    // Find files named `icon.<ext>` where <ext> is a format `image` can read.
    let entries = fs::read_dir(&dir)
        .unwrap_or_else(|e| fail(format!("could not read {}: {e}", dir.display())));
    let mut candidates: Vec<PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_file())
        .filter(|p| {
            let stem_ok = p
                .file_stem()
                .and_then(|s| s.to_str())
                .is_some_and(|s| s.eq_ignore_ascii_case("icon"));
            let fmt_ok = p
                .extension()
                .and_then(ImageFormat::from_extension)
                .is_some_and(|f| f.can_read());
            stem_ok && fmt_ok
        })
        .collect();
    candidates.sort();

    let src = match candidates.as_slice() {
        [] => {
            println!("cargo:warning=no icon image found; building without a custom icon");
            return;
        }
        [one] => one.clone(),
        many => {
            let names: Vec<_> = many
                .iter()
                .filter_map(|p| p.file_name()?.to_str().map(str::to_owned))
                .collect();
            fail(format!(
                "multiple possible icons found: {}. Keep only one icon.* file.",
                names.join(", ")
            ));
        }
    };

    let img = image::open(&src)
        .unwrap_or_else(|e| fail(format!("could not read {}: {e}", src.display())));

    let out = dir.join(OUT_NAME);
    let is_png_named_icon = src.file_name().is_some_and(|n| n == OUT_NAME);

    // Already a 48x48 icon.png: nothing to do.
    if is_png_named_icon && img.dimensions() == (SIZE, SIZE) {
        return;
    }

    let resized = if img.dimensions() == (SIZE, SIZE) {
        img
    } else {
        img.resize_to_fill(SIZE, SIZE, FilterType::Lanczos3)
    };

    resized
        .save_with_format(&out, ImageFormat::Png)
        .unwrap_or_else(|e| fail(format!("could not write {}: {e}", out.display())));

    // Remove the original (e.g. icon.jpg) so only one icon file remains.
    // Skip when it's the same file as the output (case-insensitive filesystems).
    if fs::canonicalize(&src).ok() != fs::canonicalize(&out).ok() {
        if let Err(e) = fs::remove_file(&src) {
            println!(
                "cargo:warning=wrote {OUT_NAME} but could not remove {}: {e}",
                src.display()
            );
        }
    }

    println!(
        "cargo:warning=converted {} to {OUT_NAME} ({SIZE}x{SIZE})",
        src.file_name().and_then(|n| n.to_str()).unwrap_or("icon")
    );
}
