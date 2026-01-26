use std::collections::HashMap;
use std::path::PathBuf;

pub fn resolve_name_conflicts(files: &[PathBuf]) -> Vec<(PathBuf, String)> {
    let mut name_counts: HashMap<String, Vec<PathBuf>> = HashMap::new();

    for file_path in files {
        let original_name = file_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        name_counts
            .entry(original_name)
            .or_default()
            .push(file_path.clone());
    }

    let mut result = Vec::new();

    for (original_name, paths) in name_counts {
        if paths.len() == 1 {
            result.push((paths[0].clone(), original_name));
        } else {
            let base_name = get_file_stem(&original_name);
            let extension = get_file_extension(&original_name);

            for (index, path) in paths.iter().enumerate() {
                let final_name = if index == 0 {
                    original_name.clone()
                } else {
                    let parent_name = path
                        .parent()
                        .and_then(|p| p.file_name())
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();

                    let suffix = if parent_name.is_empty() {
                        format!("_{}", index)
                    } else {
                        format!("_{}", parent_name)
                    };

                    if extension.is_empty() {
                        format!("{}{}", base_name, suffix)
                    } else {
                        format!("{}{}.{}", base_name, suffix, extension)
                    }
                };

                result.push((path.clone(), final_name));
            }
        }
    }

    result
}

pub fn get_file_stem(filename: &str) -> &str {
    if let Some(pos) = filename.rfind('.') {
        &filename[..pos]
    } else {
        filename
    }
}

pub fn get_file_extension(filename: &str) -> String {
    if let Some(pos) = filename.rfind('.') {
        filename[pos + 1..].to_string()
    } else {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_file_stem() {
        assert_eq!(get_file_stem("file.txt"), "file");
        assert_eq!(get_file_stem("archive.tar.gz"), "archive.tar");
        assert_eq!(get_file_stem("no_extension"), "no_extension");
        assert_eq!(get_file_stem(".hidden"), "");
    }

    #[test]
    fn test_get_file_extension() {
        assert_eq!(get_file_extension("file.txt"), "txt");
        assert_eq!(get_file_extension("archive.tar.gz"), "gz");
        assert_eq!(get_file_extension("no_extension"), "");
        assert_eq!(get_file_extension(".hidden"), "hidden");
    }

    #[test]
    fn test_resolve_no_conflicts() {
        let files = vec![
            PathBuf::from("/test/file1.txt"),
            PathBuf::from("/test/file2.txt"),
        ];
        let result = resolve_name_conflicts(&files);

        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|(_, name)| name == "file1.txt"));
        assert!(result.iter().any(|(_, name)| name == "file2.txt"));
    }

    #[test]
    fn test_resolve_with_conflicts() {
        let files = vec![
            PathBuf::from("/test/dir1/file.txt"),
            PathBuf::from("/test/dir2/file.txt"),
        ];
        let result = resolve_name_conflicts(&files);

        assert_eq!(result.len(), 2);

        let names: Vec<String> = result.iter().map(|(_, name)| name.clone()).collect();
        assert!(names.contains(&"file.txt".to_string()));
        assert!(
            names.contains(&"file_dir2.txt".to_string())
                || names.contains(&"file_dir1.txt".to_string())
        );
    }
}
