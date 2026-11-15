use std::{
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::{self, Command},
};

const SIZE: u32 = 48;
const OUT_NAME: &str = "icon.png";

/// Extensions considered when looking for an `icon.<ext>` file.
const EXTENSIONS: &[&str] = &[
    "png", "jpg", "jpeg", "gif", "bmp", "webp", "tif", "tiff", "ico", "tga", "qoi", "avif", "svg",
];

fn fail(msg: &str) -> ! {
    eprintln!("error: {msg}");
    process::exit(1);
}

/// Reads the width and height out of a PNG's IHDR chunk, so an icon that is
/// already the right size doesn't need `ImageMagick` at all.
fn png_dimensions(path: &Path) -> Option<(u32, u32)> {
    let mut header = [0u8; 24];
    fs::File::open(path).ok()?.read_exact(&mut header).ok()?;
    if &header[..8] != b"\x89PNG\r\n\x1a\n" || &header[12..16] != b"IHDR" {
        return None;
    }
    let width = u32::from_be_bytes(header[16..20].try_into().ok()?);
    let height = u32::from_be_bytes(header[20..24].try_into().ok()?);
    Some((width, height))
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    // After processing, the icon is always icon.png. If it's missing, Cargo reruns
    // this script each build, so a newly added icon gets picked up.
    println!("cargo:rerun-if-changed={OUT_NAME}");

    let dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    // Find files named `icon.<ext>` where <ext> is a known image format.
    let entries = fs::read_dir(&dir)
        .unwrap_or_else(|e| fail(&format!("could not read {}: {e}", dir.display())));
    let mut candidates: Vec<PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_file())
        .filter(|p| {
            let stem_ok = p
                .file_stem()
                .and_then(|s| s.to_str())
                .is_some_and(|s| s.eq_ignore_ascii_case("icon"));
            let ext_ok = p
                .extension()
                .and_then(|e| e.to_str())
                .is_some_and(|e| EXTENSIONS.iter().any(|x| e.eq_ignore_ascii_case(x)));
            stem_ok && ext_ok
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
            fail(&format!(
                "multiple possible icons found: {}. Keep only one icon.* file.",
                names.join(", ")
            ));
        }
    };

    let src_name = src.file_name().and_then(|n| n.to_str()).unwrap_or("icon");
    let out = dir.join(OUT_NAME);

    // Already a 48x48 icon.png: nothing to do.
    if src_name == OUT_NAME && png_dimensions(&src) == Some((SIZE, SIZE)) {
        return;
    }

    // Scale to cover a SIZE x SIZE square, then crop the overflow from the center.
    // Runs inside the project directory with bare file names, so nothing in the
    // directory's path can be mistaken for ImageMagick syntax.
    let size = format!("{SIZE}x{SIZE}");
    let status = Command::new("magick")
        .current_dir(&dir)
        .arg(format!("{src_name}[0]")) // first frame only, for animations and multi-size .ico
        .args(["-filter", "Lanczos", "-resize", &format!("{size}^")])
        .args(["-gravity", "center", "-extent", &size])
        .arg("-strip")
        .arg(format!("PNG32:{OUT_NAME}"))
        .status()
        .unwrap_or_else(|e| {
            fail(&format!(
                "could not run `magick` to convert {src_name}: {e}. \
                 Install ImageMagick 7, or provide a {size} {OUT_NAME} yourself."
            ))
        });
    if !status.success() {
        fail(&format!("`magick` failed to convert {src_name} ({status})"));
    }

    // Remove the original (e.g. icon.jpg) so only one icon file remains.
    // Skip when it's the same file as the output (case-insensitive filesystems).
    if fs::canonicalize(&src).ok() != fs::canonicalize(&out).ok()
        && let Err(e) = fs::remove_file(&src)
    {
        println!("cargo:warning=wrote {OUT_NAME} but could not remove {src_name}: {e}");
    }

    println!("cargo:warning=converted {src_name} to {OUT_NAME} ({size})");
}
