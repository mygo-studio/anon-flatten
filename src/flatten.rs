use crate::conflict::resolve_name_conflicts;
use crate::error::{FlattenError, Result};
use crate::file_ops;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// 扁平化配置
pub struct FlattenConfig {
    /// 源目录
    pub input: PathBuf,

    /// 目标目录
    pub output: PathBuf,

    /// 是否为预览模式
    pub preview: bool,

    /// 是否为剪切模式
    pub cut: bool,

    /// 排除的文件扩展名
    pub exclude_extensions: Vec<String>,
}

impl FlattenConfig {
    pub fn validate(&self) -> Result<()> {
        if !self.input.exists() {
            return Err(FlattenError::SourceNotFound(
                self.input.display().to_string(),
            ));
        }

        if !self.input.is_dir() {
            return Err(FlattenError::SourceNotDirectory(
                self.input.display().to_string(),
            ));
        }

        if self.output.starts_with(&self.input) {
            return Err(FlattenError::TargetInsideSource);
        }

        if !self.preview && !self.output.exists() {
            std::fs::create_dir_all(&self.output).map_err(|_| {
                FlattenError::CreateTargetDirFailed(self.output.display().to_string())
            })?;
        }

        Ok(())
    }
}

pub fn collect_files(input_dir: &Path, exclude_extensions: &[String]) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in WalkDir::new(input_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path();

            if let Some(extension) = path.extension() {
                let ext = extension.to_string_lossy().to_lowercase();
                if exclude_extensions.iter().any(|e| e.to_lowercase() == ext) {
                    continue;
                }
            }

            files.push(path.to_path_buf());
        }
    }

    Ok(files)
}

pub fn execute_flatten<F>(config: &FlattenConfig, mut progress_callback: Option<F>) -> Result<usize>
where
    F: FnMut(&str, usize, usize),
{
    let files = collect_files(&config.input, &config.exclude_extensions)?;

    if files.is_empty() {
        return Ok(0);
    }

    let name_map = resolve_name_conflicts(&files);
    let total = name_map.len();

    for (index, (source_path, target_name)) in name_map.iter().enumerate() {
        let target_path = config.output.join(target_name);

        if let Some(ref mut callback) = progress_callback {
            callback(target_name, index + 1, total);
        }

        if !config.preview {
            if config.cut {
                file_ops::move_file(source_path, &target_path)?;
            } else {
                file_ops::copy_file(source_path, &target_path)?;
            }
        }
    }

    Ok(total)
}

pub fn preview_operations(config: &FlattenConfig) -> Result<Vec<(PathBuf, String)>> {
    let files = collect_files(&config.input, &config.exclude_extensions)?;
    Ok(resolve_name_conflicts(&files))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
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
    fn test_collect_files() {
        let (_temp_dir, source_dir) = create_test_structure();
        let files = collect_files(&source_dir, &[]).unwrap();
        assert_eq!(files.len(), 7);

        let file_names: Vec<String> = files
            .iter()
            .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
            .collect();

        assert!(file_names.contains(&"file1.txt".to_string()));
        assert_eq!(
            file_names
                .iter()
                .filter(|&name| name == "file1.txt")
                .count(),
            2
        );
    }

    #[test]
    fn test_collect_files_with_exclusion() {
        let (_temp_dir, source_dir) = create_test_structure();

        // 排除 .txt 文件
        let files = collect_files(&source_dir, &["txt".to_string()]).unwrap();
        assert_eq!(files.len(), 4); // 7 - 3个txt文件 = 4

        let file_names: Vec<String> = files
            .iter()
            .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
            .collect();

        assert!(!file_names.iter().any(|name| name.ends_with(".txt")));
        assert!(file_names.contains(&"report.pdf".to_string()));
        assert!(file_names.contains(&"photo.jpg".to_string()));
    }

    #[test]
    fn test_flatten_config_validation() {
        let temp_dir = TempDir::new().unwrap();
        let source_dir = temp_dir.path().join("source");
        let target_dir = temp_dir.path().join("target");

        fs::create_dir_all(&source_dir).unwrap();

        let config = FlattenConfig {
            input: source_dir.clone(),
            output: target_dir.clone(),
            preview: false,
            cut: false,
            exclude_extensions: vec![],
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_flatten_config_target_inside_source() {
        let temp_dir = TempDir::new().unwrap();
        let source_dir = temp_dir.path().join("source");
        let target_dir = source_dir.join("target");

        fs::create_dir_all(&source_dir).unwrap();

        let config = FlattenConfig {
            input: source_dir,
            output: target_dir,
            preview: false,
            cut: false,
            exclude_extensions: vec![],
        };

        assert!(matches!(
            config.validate(),
            Err(FlattenError::TargetInsideSource)
        ));
    }

    #[test]
    fn test_execute_flatten_copy() {
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

        // 源文件应该还在
        assert!(source_dir.join("file1.txt").exists());
    }

    #[test]
    fn test_preview_operations() {
        let (_temp_dir, source_dir) = create_test_structure();
        let target_dir = _temp_dir.path().join("target");

        let config = FlattenConfig {
            input: source_dir,
            output: target_dir,
            preview: true,
            cut: false,
            exclude_extensions: vec![],
        };

        let operations = preview_operations(&config).unwrap();
        assert_eq!(operations.len(), 7);

        let target_names: Vec<String> = operations.iter().map(|(_, name)| name.clone()).collect();

        assert!(target_names.contains(&"file1.txt".to_string()));
        assert!(target_names.contains(&"report.pdf".to_string()));
    }
}
