use anon_flatten::flatten::{execute_flatten, preview_operations, FlattenConfig};
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

fn create_test_structure() -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let source_dir = temp_dir.path().join("source");

    fs::create_dir_all(&source_dir).unwrap();
    fs::create_dir_all(source_dir.join("docs/notes")).unwrap();
    fs::create_dir_all(source_dir.join("images/screenshots")).unwrap();
    fs::create_dir_all(source_dir.join("duplicate")).unwrap();

    fs::write(source_dir.join("file1.txt"), "content1").unwrap();
    fs::write(source_dir.join("docs/report.pdf"), "pdf content").unwrap();
    fs::write(source_dir.join("docs/notes/meeting.txt"), "meeting notes").unwrap();
    fs::write(source_dir.join("images/photo.jpg"), "photo data").unwrap();
    fs::write(
        source_dir.join("images/screenshots/screen1.png"),
        "screenshot1",
    )
    .unwrap();
    fs::write(
        source_dir.join("images/screenshots/screen2.png"),
        "screenshot2",
    )
    .unwrap();
    fs::write(source_dir.join("duplicate/file1.txt"), "duplicate content").unwrap();

    (temp_dir, source_dir)
}

#[test]
fn test_end_to_end_copy_mode() {
    let (_temp_dir, source_dir) = create_test_structure();
    let target_dir = _temp_dir.path().join("target");

    let config = FlattenConfig {
        input: source_dir.clone(),
        output: target_dir.clone(),
        preview: false,
        cut: false,
        exclude_extensions: vec![],
    };

    config.validate().unwrap();
    let count = execute_flatten(&config, None::<fn(&str, usize, usize)>).unwrap();
    assert_eq!(count, 7);
    assert!(target_dir.join("file1.txt").exists());
    assert!(target_dir.join("report.pdf").exists());
    assert!(target_dir.join("meeting.txt").exists());
    assert!(target_dir.join("photo.jpg").exists());
    assert!(target_dir.join("screen1.png").exists());
    assert!(target_dir.join("screen2.png").exists());

    assert!(source_dir.join("file1.txt").exists());
    assert!(source_dir.join("docs/report.pdf").exists());
    assert!(source_dir.join("duplicate/file1.txt").exists());

    let target_files: Vec<String> = fs::read_dir(&target_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();

    let file1_count = target_files
        .iter()
        .filter(|name| name.starts_with("file1"))
        .count();
    assert_eq!(file1_count, 2);
}

#[test]
fn test_end_to_end_cut_mode() {
    let (_temp_dir, source_dir) = create_test_structure();
    let target_dir = _temp_dir.path().join("target");

    let config = FlattenConfig {
        input: source_dir.clone(),
        output: target_dir.clone(),
        preview: false,
        cut: true,
        exclude_extensions: vec![],
    };

    config.validate().unwrap();
    let count = execute_flatten(&config, None::<fn(&str, usize, usize)>).unwrap();

    assert_eq!(count, 7);
    assert!(target_dir.join("file1.txt").exists());
    assert!(target_dir.join("report.pdf").exists());
    assert!(!source_dir.join("file1.txt").exists());
    assert!(!source_dir.join("docs/report.pdf").exists());
}

#[test]
fn test_preview_mode() {
    let (_temp_dir, source_dir) = create_test_structure();
    let target_dir = _temp_dir.path().join("target");

    let config = FlattenConfig {
        input: source_dir.clone(),
        output: target_dir.clone(),
        preview: true,
        cut: false,
        exclude_extensions: vec![],
    };

    let operations = preview_operations(&config).unwrap();

    assert_eq!(operations.len(), 7);
    assert!(!target_dir.exists());
    assert!(source_dir.join("file1.txt").exists());
    assert!(source_dir.join("docs/report.pdf").exists());
}

#[test]
fn test_name_conflict_resolution() {
    let (_temp_dir, source_dir) = create_test_structure();
    let target_dir = _temp_dir.path().join("target");

    let config = FlattenConfig {
        input: source_dir,
        output: target_dir.clone(),
        preview: false,
        cut: false,
        exclude_extensions: vec![],
    };

    config.validate().unwrap();
    execute_flatten(&config, None::<fn(&str, usize, usize)>).unwrap();

    let target_files: Vec<String> = fs::read_dir(&target_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();

    let mut sorted_files = target_files.clone();
    sorted_files.sort();
    sorted_files.dedup();
    assert_eq!(sorted_files.len(), target_files.len());
    assert!(target_files.contains(&"file1.txt".to_string()));
    assert!(target_files
        .iter()
        .any(|name| name.starts_with("file1_") && name.ends_with(".txt")));
}

#[test]
fn test_empty_directory() {
    let temp_dir = TempDir::new().unwrap();
    let source_dir = temp_dir.path().join("empty_source");
    let target_dir = temp_dir.path().join("target");
    fs::create_dir_all(&source_dir).unwrap();

    let config = FlattenConfig {
        input: source_dir,
        output: target_dir,
        preview: false,
        cut: false,
        exclude_extensions: vec![],
    };

    config.validate().unwrap();
    let count = execute_flatten(&config, None::<fn(&str, usize, usize)>).unwrap();

    assert_eq!(count, 0);
}

#[test]
fn test_progress_callback() {
    let (_temp_dir, source_dir) = create_test_structure();
    let target_dir = _temp_dir.path().join("target");

    let config = FlattenConfig {
        input: source_dir,
        output: target_dir,
        preview: false,
        cut: false,
        exclude_extensions: vec![],
    };

    config.validate().unwrap();

    let mut callback_count = 0;
    let callback = |_filename: &str, _current: usize, _total: usize| callback_count += 1;

    execute_flatten(&config, Some(callback)).unwrap();
    assert_eq!(callback_count, 7);
}

#[test]
fn test_exclude_extensions() {
    let (_temp_dir, source_dir) = create_test_structure();
    let target_dir = _temp_dir.path().join("target");

    let config = FlattenConfig {
        input: source_dir.clone(),
        output: target_dir.clone(),
        preview: false,
        cut: false,
        exclude_extensions: vec!["txt".to_string()],
    };

    config.validate().unwrap();
    let count = execute_flatten(&config, None::<fn(&str, usize, usize)>).unwrap();

    assert_eq!(count, 4);
    assert!(!target_dir.join("file1.txt").exists());
    assert!(!target_dir.join("meeting.txt").exists());
    assert!(target_dir.join("report.pdf").exists());
    assert!(target_dir.join("photo.jpg").exists());
    assert!(target_dir.join("screen1.png").exists());
    assert!(target_dir.join("screen2.png").exists());
}

#[test]
fn test_exclude_multiple_extensions() {
    let (_temp_dir, source_dir) = create_test_structure();
    let target_dir = _temp_dir.path().join("target");

    let config = FlattenConfig {
        input: source_dir,
        output: target_dir.clone(),
        preview: false,
        cut: false,
        exclude_extensions: vec!["txt".to_string(), "png".to_string()],
    };

    config.validate().unwrap();
    let count = execute_flatten(&config, None::<fn(&str, usize, usize)>).unwrap();

    assert_eq!(count, 2);
    assert!(target_dir.join("report.pdf").exists());
    assert!(target_dir.join("photo.jpg").exists());
}

#[test]
fn test_exclude_case_insensitive() {
    let temp_dir = TempDir::new().unwrap();
    let source_dir = temp_dir.path().join("source");
    let target_dir = temp_dir.path().join("target");

    fs::create_dir_all(&source_dir).unwrap();
    fs::write(source_dir.join("file.TXT"), "content").unwrap();
    fs::write(source_dir.join("file.txt"), "content").unwrap();
    fs::write(source_dir.join("file.Txt"), "content").unwrap();
    fs::write(source_dir.join("file.pdf"), "content").unwrap();

    let config = FlattenConfig {
        input: source_dir,
        output: target_dir.clone(),
        preview: false,
        cut: false,
        exclude_extensions: vec!["txt".to_string()],
    };

    config.validate().unwrap();
    let count = execute_flatten(&config, None::<fn(&str, usize, usize)>).unwrap();

    assert_eq!(count, 1);
    assert!(target_dir.join("file.pdf").exists());
}

#[test]
fn test_exclude_comma_separated_simulation() {
    let (_temp_dir, source_dir) = create_test_structure();
    let target_dir = _temp_dir.path().join("target");

    let config = FlattenConfig {
        input: source_dir,
        output: target_dir.clone(),
        preview: false,
        cut: false,
        exclude_extensions: vec!["txt".to_string(), "png".to_string()],
    };

    config.validate().unwrap();
    let count = execute_flatten(&config, None::<fn(&str, usize, usize)>).unwrap();

    assert_eq!(count, 2);
    assert!(target_dir.join("report.pdf").exists());
    assert!(target_dir.join("photo.jpg").exists());
}
