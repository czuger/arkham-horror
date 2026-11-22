//! Shrinks the illustrations before they are built into the program.
//!
//! The originals in `illustrations/` weigh about 2 MB each (186 MB in all).
//! Each one becomes two JPEG files under `$OUT_DIR/illustrations/`:
//! `grand/<path>.jpg` for the scene and `vignette/<path>.jpg` for thumbnails.
//! It also writes `$OUT_DIR/illustration_files.rs`, the table of those files
//! (path, bytes, fingerprint) that `src/illustrations.rs` builds in.
//! Cargo reruns this only when the `illustrations/` folder changes.

use std::error::Error;
use std::fs::{self, File};
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::thread;

use image::ImageReader;
use image::codecs::jpeg::JpegEncoder;
use image::imageops::FilterType;

/// One output size: folder name, longest side in pixels, JPEG quality.
struct Size {
    folder: &'static str,
    longest_side: u32,
    quality: u8,
}

const SIZES: [Size; 2] = [
    Size {
        folder: "grand",
        longest_side: 1280,
        quality: 82,
    },
    Size {
        folder: "vignette",
        longest_side: 320,
        quality: 78,
    },
];

fn main() -> Result<(), Box<dyn Error>> {
    let source = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../illustrations");
    let target = PathBuf::from(std::env::var("OUT_DIR")?).join("illustrations");
    println!("cargo::rerun-if-changed={}", source.display());
    let mut pictures = Vec::new();
    list_pictures(&source, Path::new(""), &mut pictures)?;
    shrink_all(&source, &target, &pictures)?;
    write_table(&target, &pictures)
}

/// Writes the table of the shrunk files, sorted by path, for `include!`.
fn write_table(target: &Path, pictures: &[PathBuf]) -> Result<(), Box<dyn Error>> {
    let mut rows = Vec::new();
    for size in &SIZES {
        for picture in pictures {
            let relative = Path::new(size.folder).join(picture).with_extension("jpg");
            let key = relative.to_string_lossy().replace('\\', "/");
            let file = target.join(&relative);
            let fingerprint = fnv1a(&fs::read(&file)?);
            rows.push((key, file, fingerprint));
        }
    }
    rows.sort();
    let mut table = String::from("&[\n");
    for (key, file, fingerprint) in rows {
        let line = format!(
            "    ({key:?}, include_bytes!({:?}), \"{fingerprint:016x}\"),\n",
            file.to_string_lossy()
        );
        table.push_str(&line);
    }
    table.push(']');
    let out = PathBuf::from(std::env::var("OUT_DIR")?).join("illustration_files.rs");
    fs::write(out, table)?;
    Ok(())
}

/// FNV-1a, 64 bits: a fingerprint that changes with the file.
fn fnv1a(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x0100_0000_01b3)
    })
}

/// Collects the PNG files under `folder`, relative to the source root.
fn list_pictures(
    root: &Path,
    folder: &Path,
    found: &mut Vec<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(root.join(folder))? {
        let entry = entry?;
        let relative = folder.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            list_pictures(root, &relative, found)?;
        } else if relative.extension().is_some_and(|ext| ext == "png") {
            println!("cargo::rerun-if-changed={}", root.join(&relative).display());
            found.push(relative);
        }
    }
    Ok(())
}

/// Shrinks every picture, spread over the available cores.
fn shrink_all(source: &Path, target: &Path, pictures: &[PathBuf]) -> Result<(), Box<dyn Error>> {
    let workers = thread::available_parallelism().map_or(4, usize::from);
    let chunk = pictures.len().div_ceil(workers).max(1);
    thread::scope(|scope| {
        let handles: Vec<_> = pictures
            .chunks(chunk)
            .map(|part| scope.spawn(move || shrink_each(source, target, part)))
            .collect();
        handles
            .into_iter()
            .map(|handle| {
                handle
                    .join()
                    .map_err(|_| "a worker thread panicked".to_owned())
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .collect::<Result<(), String>>()
    })?;
    Ok(())
}

fn shrink_each(source: &Path, target: &Path, pictures: &[PathBuf]) -> Result<(), String> {
    for picture in pictures {
        shrink(source, target, picture)
            .map_err(|error| format!("{}: {error}", picture.display()))?;
    }
    Ok(())
}

/// Writes both sizes of one picture.
fn shrink(source: &Path, target: &Path, picture: &Path) -> Result<(), Box<dyn Error>> {
    let original = ImageReader::open(source.join(picture))?.decode()?;
    for size in &SIZES {
        let output = target.join(size.folder).join(picture).with_extension("jpg");
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent)?;
        }
        let side = size.longest_side;
        let small = original
            .resize(side, side, FilterType::CatmullRom)
            .to_rgb8();
        let writer = BufWriter::new(File::create(&output)?);
        JpegEncoder::new_with_quality(writer, size.quality).encode_image(&small)?;
    }
    Ok(())
}
