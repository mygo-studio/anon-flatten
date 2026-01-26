use crate::error::{FlattenError, Result};
use std::path::Path;

pub fn move_file(source: &Path, target: &Path) -> Result<()> {
    if target.exists() {
        std::fs::remove_file(target)
            .map_err(|_| FlattenError::RemoveExistingFileFailed(target.display().to_string()))?;
    }

    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|_| FlattenError::CreateTargetDirFailed(parent.display().to_string()))?;
    }

    std::fs::rename(source, target).map_err(|_| {
        FlattenError::MoveFileFailed(source.display().to_string(), target.display().to_string())
    })?;

    Ok(())
}

pub fn copy_file(source: &Path, target: &Path) -> Result<()> {
    if target.exists() {
        std::fs::remove_file(target)
            .map_err(|_| FlattenError::RemoveExistingFileFailed(target.display().to_string()))?;
    }

    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|_| FlattenError::CreateTargetDirFailed(parent.display().to_string()))?;
    }

    std::fs::copy(source, target).map_err(|_| {
        FlattenError::CopyFileFailed(source.display().to_string(), target.display().to_string())
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_copy_file() {
        let temp_dir = TempDir::new().unwrap();
        let source_file = temp_dir.path().join("source.txt");
        let target_file = temp_dir.path().join("target.txt");

        fs::write(&source_file, "test content").unwrap();
        copy_file(&source_file, &target_file).unwrap();

        assert!(target_file.exists());
        assert!(source_file.exists());
        assert_eq!(fs::read_to_string(&target_file).unwrap(), "test content");
    }

    #[test]
    fn test_move_file() {
        let temp_dir = TempDir::new().unwrap();
        let source_file = temp_dir.path().join("source.txt");
        let target_file = temp_dir.path().join("target.txt");

        fs::write(&source_file, "test content").unwrap();
        move_file(&source_file, &target_file).unwrap();

        assert!(target_file.exists());
        assert!(!source_file.exists());
        assert_eq!(fs::read_to_string(&target_file).unwrap(), "test content");
    }

    #[test]
    fn test_copy_file_with_subdirectory() {
        let temp_dir = TempDir::new().unwrap();
        let source_file = temp_dir.path().join("source.txt");
        let target_file = temp_dir.path().join("subdir/target.txt");

        fs::write(&source_file, "test content").unwrap();
        copy_file(&source_file, &target_file).unwrap();

        assert!(target_file.exists());
        assert!(source_file.exists());
    }

    #[test]
    fn test_overwrite_existing_file() {
        let temp_dir = TempDir::new().unwrap();
        let source_file = temp_dir.path().join("source.txt");
        let target_file = temp_dir.path().join("target.txt");

        fs::write(&source_file, "new content").unwrap();
        fs::write(&target_file, "old content").unwrap();

        copy_file(&source_file, &target_file).unwrap();

        assert_eq!(fs::read_to_string(&target_file).unwrap(), "new content");
    }
}
