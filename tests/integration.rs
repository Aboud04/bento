use assert_cmd::Command;
use predicates::prelude::*;
use serial_test::serial;

fn bento() -> Command {
    Command::cargo_bin("bento").unwrap()
}

// ─── Help & Version ──────────────────────────────────────────

#[test]
#[serial]
fn help_shows_usage() {
    bento()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage"));
}

#[test]
#[serial]
fn version_shows_version() {
    bento()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("bento"));
}

#[test]
#[serial]
fn no_args_shows_help() {
    bento()
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

// ─── List ────────────────────────────────────────────────────

#[test]
#[serial]
fn list_empty_vault() {
    bento()
        .arg("list")
        .assert()
        .success();
}

// ─── Search ──────────────────────────────────────────────────

#[test]
#[serial]
fn search_nonexistent_project() {
    bento()
        .args(["search", "this-project-definitely-does-not-exist-xyz"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Not found"));
}

// ─── Stats ───────────────────────────────────────────────────

#[test]
#[serial]
fn stats_empty_vault() {
    bento()
        .arg("stats")
        .assert()
        .success();
}

// ─── Config ──────────────────────────────────────────────────

#[test]
#[serial]
fn config_show_default() {
    bento()
        .arg("config")
        .assert()
        .success()
        .stdout(predicate::str::contains("default_algo"));
}

#[test]
#[serial]
fn config_set_valid_algo() {
    bento()
        .args(["config", "--algo", "gzip"])
        .assert()
        .success()
        .stdout(predicate::str::contains("gzip"));

    // Reset back to zstd
    bento()
        .args(["config", "--algo", "zstd"])
        .assert()
        .success();
}

#[test]
#[serial]
fn config_set_invalid_algo() {
    bento()
        .args(["config", "--algo", "fakealgo"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unknown algorithm"));
}

// ─── Delete nonexistent ──────────────────────────────────────

#[test]
#[serial]
fn delete_nonexistent_project() {
    bento()
        .args(["delete", "nonexistent-project-xyz"])
        .assert()
        .failure();
}

// ─── Unpack nonexistent ──────────────────────────────────────

#[test]
#[serial]
fn unpack_nonexistent_project() {
    bento()
        .args(["unpack", "nonexistent-project-xyz"])
        .assert()
        .failure();
}

// ─── Pack + Unpack + Delete round-trip ───────────────────────

#[test]
#[serial]
fn pack_unpack_delete_roundtrip() {
    // Create a temp project directory
    let tmp = tempfile::tempdir().unwrap();
    let project_dir = tmp.path().join("test-project-roundtrip");
    std::fs::create_dir(&project_dir).unwrap();
    std::fs::write(project_dir.join("README.md"), "# Test Project").unwrap();
    std::fs::write(project_dir.join("main.rs"), "fn main() {}").unwrap();

    // Pack it
    bento()
        .args(["pack", "v1.0", "--force"])
        .current_dir(&project_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Done!"));

    // Original dir should be deleted
    assert!(!project_dir.exists());

    // Should appear in list
    bento()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("test-project-roundtrip"));

    // Search by name
    bento()
        .args(["search", "test-project-roundtrip"])
        .assert()
        .success()
        .stdout(predicate::str::contains("test-project-roundtrip"));

    // Search by tag
    bento()
        .args(["search", "v1.0"])
        .assert()
        .success()
        .stdout(predicate::str::contains("test-project-roundtrip"));

    // Stats should show something
    bento()
        .arg("stats")
        .assert()
        .success()
        .stdout(predicate::str::contains("Total archives"));

    // Unpack it
    bento()
        .args(["unpack", "test-project-roundtrip"])
        .assert()
        .success()
        .stdout(predicate::str::contains("__bento_cd:"));

    // Delete it from vault
    bento()
        .args(["delete", "test-project-roundtrip"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Removed"));

    // Should no longer appear
    bento()
        .args(["search", "test-project-roundtrip"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Not found"));
}

// ─── Pack with specific algorithms ──────────────────────────

fn pack_with_algo(algo: &str) {
    let tmp = tempfile::tempdir().unwrap();
    let name = format!("test-algo-{algo}");
    let project_dir = tmp.path().join(&name);
    std::fs::create_dir(&project_dir).unwrap();
    std::fs::write(project_dir.join("data.txt"), "some test data for compression").unwrap();

    bento()
        .args(["pack", "test", "--algo", algo, "--force"])
        .current_dir(&project_dir)
        .assert()
        .success();

    // Clean up: delete from vault
    bento()
        .args(["delete", &name])
        .assert()
        .success();
}

#[test]
#[serial]
fn pack_with_zstd() {
    pack_with_algo("zstd");
}

#[test]
#[serial]
fn pack_with_gzip() {
    pack_with_algo("gzip");
}

#[test]
#[serial]
fn pack_with_bzip2() {
    pack_with_algo("bzip2");
}

#[test]
#[serial]
fn pack_with_xz() {
    pack_with_algo("xz");
}

#[test]
#[serial]
fn pack_with_lz4() {
    pack_with_algo("lz4");
}

#[test]
#[serial]
fn pack_with_snappy() {
    pack_with_algo("snappy");
}

#[test]
#[serial]
fn pack_with_brotli() {
    pack_with_algo("brotli");
}

// ─── Edge cases ──────────────────────────────────────────────

#[test]
#[serial]
fn pack_help_shows_options() {
    bento()
        .args(["pack", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--algo"))
        .stdout(predicate::str::contains("--force"))
        .stdout(predicate::str::contains("--repo"));
}

#[test]
#[serial]
fn unpack_by_tag() {
    let tmp = tempfile::tempdir().unwrap();
    let project_dir = tmp.path().join("test-tag-lookup");
    std::fs::create_dir(&project_dir).unwrap();
    std::fs::write(project_dir.join("file.txt"), "tag test").unwrap();

    bento()
        .args(["pack", "unique-tag-12345", "--force"])
        .current_dir(&project_dir)
        .assert()
        .success();

    // Unpack by tag instead of name
    bento()
        .args(["unpack", "unique-tag-12345"])
        .assert()
        .success()
        .stdout(predicate::str::contains("__bento_cd:"));

    // Clean up
    bento()
        .args(["delete", "test-tag-lookup"])
        .assert()
        .success();
}

// ─── Rename ──────────────────────────────────────────────────

#[test]
#[serial]
fn rename_project() {
    let tmp = tempfile::tempdir().unwrap();
    let project_dir = tmp.path().join("test-rename-src");
    std::fs::create_dir(&project_dir).unwrap();
    std::fs::write(project_dir.join("file.txt"), "rename me").unwrap();

    bento()
        .args(["pack", "v1", "--force"])
        .current_dir(&project_dir)
        .assert()
        .success();

    bento()
        .args(["rename", "test-rename-src", "test-rename-dst"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Renamed"));

    // Old name should not be found
    bento()
        .args(["search", "test-rename-src"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Not found"));

    // New name should be found
    bento()
        .args(["search", "test-rename-dst"])
        .assert()
        .success()
        .stdout(predicate::str::contains("test-rename-dst"));

    // Clean up
    bento().args(["delete", "test-rename-dst"]).assert().success();
}

// ─── Info ────────────────────────────────────────────────────

#[test]
#[serial]
fn info_shows_details() {
    let tmp = tempfile::tempdir().unwrap();
    let project_dir = tmp.path().join("test-info-proj");
    std::fs::create_dir(&project_dir).unwrap();
    std::fs::write(project_dir.join("data.txt"), "info test").unwrap();

    bento()
        .args(["pack", "v2.0", "--force"])
        .current_dir(&project_dir)
        .assert()
        .success();

    bento()
        .args(["info", "test-info-proj"])
        .assert()
        .success()
        .stdout(predicate::str::contains("test-info-proj"))
        .stdout(predicate::str::contains("v2.0"))
        .stdout(predicate::str::contains("zstd"));

    bento().args(["delete", "test-info-proj"]).assert().success();
}

#[test]
#[serial]
fn info_nonexistent_project() {
    bento()
        .args(["info", "nonexistent-xyz"])
        .assert()
        .failure();
}

// ─── Export ──────────────────────────────────────────────────

#[test]
#[serial]
fn export_to_custom_path() {
    let tmp = tempfile::tempdir().unwrap();
    let project_dir = tmp.path().join("test-export-proj");
    std::fs::create_dir(&project_dir).unwrap();
    std::fs::write(project_dir.join("readme.md"), "export test").unwrap();

    bento()
        .args(["pack", "v1", "--force"])
        .current_dir(&project_dir)
        .assert()
        .success();

    let export_dir = tmp.path().join("exported");
    bento()
        .args(["export", "test-export-proj", export_dir.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Exported"));

    // Verify file was extracted
    assert!(export_dir.join("readme.md").exists());
    assert_eq!(
        std::fs::read_to_string(export_dir.join("readme.md")).unwrap(),
        "export test"
    );

    bento().args(["delete", "test-export-proj"]).assert().success();
}

// ─── Import ──────────────────────────────────────────────────

#[test]
#[serial]
fn import_archive() {
    // First create an archive to import
    let tmp = tempfile::tempdir().unwrap();
    let source_dir = tmp.path().join("import-source");
    std::fs::create_dir(&source_dir).unwrap();
    std::fs::write(source_dir.join("data.txt"), "import test data").unwrap();

    let archive_path = tmp.path().join("my-imported-project.tar.zst");
    // Compress directly using the library
    let file = std::fs::File::create(&archive_path).unwrap();
    let encoder = zstd::Encoder::new(file, 3).unwrap();
    let mut tar = tar::Builder::new(encoder);
    tar.append_dir_all(".", &source_dir).unwrap();
    tar.into_inner().unwrap().finish().unwrap();

    bento()
        .args(["import", archive_path.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Imported"));

    // Should appear in list
    bento()
        .args(["search", "my-imported-project"])
        .assert()
        .success()
        .stdout(predicate::str::contains("my-imported-project"));

    bento()
        .args(["delete", "my-imported-project"])
        .assert()
        .success();
}

#[test]
#[serial]
fn import_nonexistent_file() {
    bento()
        .args(["import", "/tmp/nonexistent-file-xyz.tar.zst"])
        .assert()
        .failure();
}

// ─── History ─────────────────────────────────────────────────

#[test]
#[serial]
fn history_shows_entries() {
    let tmp = tempfile::tempdir().unwrap();
    let project_dir = tmp.path().join("test-history-proj");
    std::fs::create_dir(&project_dir).unwrap();
    std::fs::write(project_dir.join("f.txt"), "history").unwrap();

    bento()
        .args(["pack", "v1", "--force"])
        .current_dir(&project_dir)
        .assert()
        .success();

    bento()
        .arg("history")
        .assert()
        .success()
        .stdout(predicate::str::contains("test-history-proj"))
        .stdout(predicate::str::contains("newest first"));

    bento().args(["delete", "test-history-proj"]).assert().success();
}

// ─── Clean ───────────────────────────────────────────────────

#[test]
#[serial]
fn clean_empty_workspace() {
    // Clean any leftover workspace items first
    let _ = bento().args(["clean", "--force"]).output();

    // Now workspace should be empty
    bento()
        .args(["clean", "--force"])
        .assert()
        .success()
        .stdout(predicate::str::contains("already clean"));
}

#[test]
#[serial]
fn clean_after_unpack() {
    let tmp = tempfile::tempdir().unwrap();
    let project_dir = tmp.path().join("test-clean-proj");
    std::fs::create_dir(&project_dir).unwrap();
    std::fs::write(project_dir.join("f.txt"), "clean me").unwrap();

    bento()
        .args(["pack", "v1", "--force"])
        .current_dir(&project_dir)
        .assert()
        .success();

    // Unpack creates workspace copy
    bento()
        .args(["unpack", "test-clean-proj"])
        .assert()
        .success();

    // Clean should remove it
    bento()
        .args(["clean", "--force"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Cleaned"));

    bento().args(["delete", "test-clean-proj"]).assert().success();
}
