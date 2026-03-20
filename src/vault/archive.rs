use std::path::Path;

/// Recursively calculates the total size of a directory in bytes.
pub fn dir_size(path: &Path) -> anyhow::Result<u64> {
    let mut total = 0;
    if path.is_dir() {
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                total += dir_size(&path)?;
            } else {
                total += entry.metadata()?.len();
            }
        }
    }
    Ok(total)
}

/// Formats a byte count into a human-readable string (e.g. "4.2 MB").
pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;
    const GB: u64 = 1024 * MB;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Compresses `source_dir` into a tar archive at `dest_file` using the given algorithm.
/// Supported algorithms: "zstd", "gzip", "bzip2", "xz", "lz4", "snappy", "brotli"
pub fn compress(source_dir: &Path, dest_file: &Path, algo: &str) -> anyhow::Result<()> {
    let file = std::fs::File::create(dest_file)?;

    match algo {
        "zstd" => {
            let encoder = zstd::Encoder::new(file, 3)?;
            let mut tar = tar::Builder::new(encoder);
            tar.append_dir_all(".", source_dir)?;
            tar.into_inner()?.finish()?;
        }
        "gzip" => {
            let encoder = flate2::write::GzEncoder::new(file, flate2::Compression::default());
            let mut tar = tar::Builder::new(encoder);
            tar.append_dir_all(".", source_dir)?;
            tar.into_inner()?.finish()?;
        }
        "bzip2" => {
            let encoder = bzip2::write::BzEncoder::new(file, bzip2::Compression::default());
            let mut tar = tar::Builder::new(encoder);
            tar.append_dir_all(".", source_dir)?;
            tar.into_inner()?.finish()?;
        }
        "xz" => {
            let encoder = xz2::write::XzEncoder::new(file, 6);
            let mut tar = tar::Builder::new(encoder);
            tar.append_dir_all(".", source_dir)?;
            tar.into_inner()?.finish()?;
        }
        "lz4" => {
            let encoder = lz4_flex::frame::FrameEncoder::new(file);
            let mut tar = tar::Builder::new(encoder);
            tar.append_dir_all(".", source_dir)?;
            tar.into_inner()?.finish().map_err(|e| anyhow::anyhow!(e))?;
        }
        "snappy" => {
            let encoder = snap::write::FrameEncoder::new(file);
            let mut tar = tar::Builder::new(encoder);
            tar.append_dir_all(".", source_dir)?;
            tar.into_inner()?.into_inner().map_err(|e| anyhow::anyhow!(e.to_string()))?;
        }
        "brotli" => {
            let encoder = brotli::CompressorWriter::new(file, 4096, 6, 22);
            let mut tar = tar::Builder::new(encoder);
            tar.append_dir_all(".", source_dir)?;
            tar.into_inner()?;
        }
        _ => return Err(anyhow::anyhow!("Unknown algorithm: {}", algo)),
    }

    Ok(())
}

/// Extracts a compressed tar archive at `archive_file` into `dest_dir`.
pub fn extract(archive_file: &Path, dest_dir: &Path, algo: &str) -> anyhow::Result<()> {
    let file = std::fs::File::open(archive_file)?;

    match algo {
        "zstd" => {
            let decoder = zstd::Decoder::new(file)?;
            tar::Archive::new(decoder).unpack(dest_dir)?;
        }
        "gzip" => {
            let decoder = flate2::read::GzDecoder::new(file);
            tar::Archive::new(decoder).unpack(dest_dir)?;
        }
        "bzip2" => {
            let decoder = bzip2::read::BzDecoder::new(file);
            tar::Archive::new(decoder).unpack(dest_dir)?;
        }
        "xz" => {
            let decoder = xz2::read::XzDecoder::new(file);
            tar::Archive::new(decoder).unpack(dest_dir)?;
        }
        "lz4" => {
            let decoder = lz4_flex::frame::FrameDecoder::new(file);
            tar::Archive::new(decoder).unpack(dest_dir)?;
        }
        "snappy" => {
            let decoder = snap::read::FrameDecoder::new(file);
            tar::Archive::new(decoder).unpack(dest_dir)?;
        }
        "brotli" => {
            let decoder = brotli::Decompressor::new(file, 4096);
            tar::Archive::new(decoder).unpack(dest_dir)?;
        }
        _ => return Err(anyhow::anyhow!("Unknown algorithm: {}", algo)),
    }

    Ok(())
}

/// Maps an algorithm name to its file extension.
pub fn get_extension(algo: &str) -> &str {
    match algo {
      "zstd"    => ".tar.zst",
      "gzip"    => ".tar.gz",
      "bzip2"   => ".tar.bz2",
      "xz"      => ".tar.xz",
      "lz4"     => ".tar.lz4",
      "snappy"  => ".tar.sz",
      "brotli"  => ".tar.br",
      _         => ".tar.zst",
    }
   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_extension_all_algorithms() {
        assert_eq!(get_extension("zstd"), ".tar.zst");
        assert_eq!(get_extension("gzip"), ".tar.gz");
        assert_eq!(get_extension("bzip2"), ".tar.bz2");
        assert_eq!(get_extension("xz"), ".tar.xz");
        assert_eq!(get_extension("lz4"), ".tar.lz4");
        assert_eq!(get_extension("snappy"), ".tar.sz");
        assert_eq!(get_extension("brotli"), ".tar.br");
    }

    #[test]
    fn get_extension_unknown_falls_back() {
        assert_eq!(get_extension("unknown"), ".tar.zst");
    }

    #[test]
    fn format_size_bytes() {
        assert_eq!(format_size(0), "0 B");
        assert_eq!(format_size(512), "512 B");
    }

    #[test]
    fn format_size_kilobytes() {
        assert_eq!(format_size(1024), "1.0 KB");
        assert_eq!(format_size(1536), "1.5 KB");
    }

    #[test]
    fn format_size_megabytes() {
        assert_eq!(format_size(1024 * 1024), "1.0 MB");
    }

    #[test]
    fn format_size_gigabytes() {
        assert_eq!(format_size(1024 * 1024 * 1024), "1.0 GB");
    }

    #[test]
    fn dir_size_empty() {
        let tmp = tempfile::tempdir().unwrap();
        assert_eq!(dir_size(tmp.path()).unwrap(), 0);
    }

    #[test]
    fn dir_size_with_files() {
        let tmp = tempfile::tempdir().unwrap();
        let f1 = tmp.path().join("a.txt");
        let f2 = tmp.path().join("b.txt");
        std::fs::write(&f1, "hello").unwrap(); // 5 bytes
        std::fs::write(&f2, "world!").unwrap(); // 6 bytes
        assert_eq!(dir_size(tmp.path()).unwrap(), 11);
    }

    fn create_test_dir() -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        let sub = tmp.path().join("subdir");
        std::fs::create_dir(&sub).unwrap();
        std::fs::write(tmp.path().join("file1.txt"), "hello world").unwrap();
        std::fs::write(sub.join("file2.txt"), "nested file content").unwrap();
        tmp
    }

    #[test]
    fn compress_extract_roundtrip_zstd() {
        roundtrip_test("zstd");
    }

    #[test]
    fn compress_extract_roundtrip_gzip() {
        roundtrip_test("gzip");
    }

    #[test]
    fn compress_extract_roundtrip_bzip2() {
        roundtrip_test("bzip2");
    }

    #[test]
    fn compress_extract_roundtrip_xz() {
        roundtrip_test("xz");
    }

    #[test]
    fn compress_extract_roundtrip_lz4() {
        roundtrip_test("lz4");
    }

    #[test]
    fn compress_extract_roundtrip_snappy() {
        roundtrip_test("snappy");
    }

    #[test]
    fn compress_extract_roundtrip_brotli() {
        roundtrip_test("brotli");
    }

    fn roundtrip_test(algo: &str) {
        let source = create_test_dir();
        let archive_dir = tempfile::tempdir().unwrap();
        let archive_path = archive_dir.path().join(format!("test{}", get_extension(algo)));

        compress(source.path(), &archive_path, algo).unwrap();
        assert!(archive_path.exists());
        assert!(archive_path.metadata().unwrap().len() > 0);

        let extract_dir = tempfile::tempdir().unwrap();
        extract(&archive_path, extract_dir.path(), algo).unwrap();

        // Verify extracted content matches original
        let extracted_file1 = extract_dir.path().join("file1.txt");
        let extracted_file2 = extract_dir.path().join("subdir").join("file2.txt");
        assert_eq!(std::fs::read_to_string(extracted_file1).unwrap(), "hello world");
        assert_eq!(std::fs::read_to_string(extracted_file2).unwrap(), "nested file content");
    }

    #[test]
    fn compress_unknown_algo_errors() {
        let source = create_test_dir();
        let archive_dir = tempfile::tempdir().unwrap();
        let archive_path = archive_dir.path().join("test.tar.unknown");
        let result = compress(source.path(), &archive_path, "fakealgo");
        assert!(result.is_err());
    }

    #[test]
    fn extract_unknown_algo_errors() {
        let tmp = tempfile::tempdir().unwrap();
        let fake_archive = tmp.path().join("fake.tar");
        std::fs::File::create(&fake_archive).unwrap();
        let result = extract(&fake_archive, tmp.path(), "fakealgo");
        assert!(result.is_err());
    }
}
