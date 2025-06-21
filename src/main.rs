use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

// MyGO!!!!! 成员代表色
const ANON_COLOR: (u8, u8, u8) = (255, 136, 153);    // 千早爱音 #FF8899 粉色
const TOMORI_COLOR: (u8, u8, u8) = (119, 187, 221);  // 高松灯 #77BBDD 蓝色
const SOYO_COLOR: (u8, u8, u8) = (255, 221, 136);    // 长崎爽世 #FFDD88 黄色
const TAKI_COLOR: (u8, u8, u8) = (119, 119, 170);    // 椎名立希 #7777AA 紫色
const RANA_COLOR: (u8, u8, u8) = (119, 221, 119);    // 要乐奈 #77DD77 绿色

#[derive(Parser)]
#[command(
    name = "anon-flatten",
    about = "一个简单的文件目录扁平化工具，让复杂的嵌套文件夹结构变得和爱音一样平 | A simple file directory flattening tool",
    version,
    after_help = "\x1b[38;2;255;136;153m🎸 让你的文件夹像千早爱音一样，简单直接，一马平川！\x1b[0m"
)]
struct Args {
    #[arg(short = 'i', long = "input", help = "源文件夹路径 | Source directory path")]
    input: PathBuf,

    #[arg(short = 'o', long = "output", help = "目标文件夹路径 | Target directory path")]
    output: PathBuf,

    #[arg(short = 'p', long = "preview", help = "预览模式 | Preview mode")]
    preview: bool,

    #[arg(short = 'x', long = "cut", help = "剪切模式 | Cut mode")]
    cut: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if !args.input.exists() {
        anyhow::bail!("{}", "❌ 源文件夹不存在 | Source directory does not exist".truecolor(TAKI_COLOR.0, TAKI_COLOR.1, TAKI_COLOR.2).bold());
    }

    if !args.input.is_dir() {
        anyhow::bail!("{}", "❌ 源路径不是文件夹 | Source path is not a directory".truecolor(TAKI_COLOR.0, TAKI_COLOR.1, TAKI_COLOR.2).bold());
    }

    if args.output.starts_with(&args.input) {
        anyhow::bail!("{}", "❌ 目标文件夹不能在源文件夹内部 | Target directory cannot be inside source directory!".truecolor(TAKI_COLOR.0, TAKI_COLOR.1, TAKI_COLOR.2).bold());
    }

    if !args.preview && !args.output.exists() {
        std::fs::create_dir_all(&args.output)
            .with_context(|| format!("{}: {}", "❌ 无法创建目标文件夹 | Failed to create target directory".truecolor(TAKI_COLOR.0, TAKI_COLOR.1, TAKI_COLOR.2).bold(), args.output.display()))?;
    }

    println!("{}", "🎸 开始扁平化操作 | Starting flatten operation...".truecolor(ANON_COLOR.0, ANON_COLOR.1, ANON_COLOR.2).bold());
    println!("{:<20} {}", "📂 源文件夹 | Source:".truecolor(TOMORI_COLOR.0, TOMORI_COLOR.1, TOMORI_COLOR.2), args.input.display().to_string().truecolor(SOYO_COLOR.0, SOYO_COLOR.1, SOYO_COLOR.2));
    println!("{:<20} {}", "📁 目标文件夹 | Target:".truecolor(TOMORI_COLOR.0, TOMORI_COLOR.1, TOMORI_COLOR.2), args.output.display().to_string().truecolor(SOYO_COLOR.0, SOYO_COLOR.1, SOYO_COLOR.2));

    let operation = if args.cut { 
        "移动 | Move".truecolor(TAKI_COLOR.0, TAKI_COLOR.1, TAKI_COLOR.2).bold() 
    } else { 
        "复制 | Copy".truecolor(RANA_COLOR.0, RANA_COLOR.1, RANA_COLOR.2).bold() 
    };
    println!("{:<20} {}", "🔄 操作模式 | Operation:".truecolor(TOMORI_COLOR.0, TOMORI_COLOR.1, TOMORI_COLOR.2), operation);

    if args.preview {
        println!("{}", "👀 预览模式 | Preview mode - operations to be performed:".truecolor(ANON_COLOR.0, ANON_COLOR.1, ANON_COLOR.2).bold());
    }

    let files = collect_files(&args.input)?;
    
    if files.is_empty() {
        println!("{}", "📭 源文件夹中没有找到任何文件 | No files found in source directory".truecolor(SOYO_COLOR.0, SOYO_COLOR.1, SOYO_COLOR.2));
        return Ok(());
    }

    println!("{:<20} {} {}", "📋 找到 | Found:".truecolor(TOMORI_COLOR.0, TOMORI_COLOR.1, TOMORI_COLOR.2), files.len().to_string().truecolor(ANON_COLOR.0, ANON_COLOR.1, ANON_COLOR.2).bold(), "个文件 | files".truecolor(TOMORI_COLOR.0, TOMORI_COLOR.1, TOMORI_COLOR.2));

    let name_map = resolve_name_conflicts(&files);
    
    // 创建进度条
    let progress_bar = if !args.preview {
        let pb = ProgressBar::new(name_map.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("🎸 {spinner:.magenta} [{elapsed_precise}] [{bar:40.magenta/white}] {pos:>7}/{len:7} {msg}")
                .unwrap()
                .progress_chars("█▇▆▅▄▃▂▁  ")
        );
        Some(pb)
    } else {
        None
    };
    
    for (source_path, target_name) in name_map {
        let target_path = args.output.join(&target_name);
        
        if args.preview {
            let op = if args.cut { 
                "移动 | Move".truecolor(TAKI_COLOR.0, TAKI_COLOR.1, TAKI_COLOR.2) 
            } else { 
                "复制 | Copy".truecolor(RANA_COLOR.0, RANA_COLOR.1, RANA_COLOR.2) 
            };
            println!("  {:<15} {} -> {}", op, source_path.display().to_string().white().dimmed(), target_name.truecolor(ANON_COLOR.0, ANON_COLOR.1, ANON_COLOR.2));
        } else {
            if let Some(ref pb) = progress_bar {
                let display_name = if target_name.len() > 30 {
                    format!("{}...", &target_name[..27])
                } else {
                    target_name.clone()
                };
                pb.set_message(display_name);
            }
            
            if args.cut {
                move_file(&source_path, &target_path)?;
                if progress_bar.is_none() {
                    println!("{} {} -> {}", "✂️".truecolor(TAKI_COLOR.0, TAKI_COLOR.1, TAKI_COLOR.2), source_path.display().to_string().white().dimmed(), target_name.truecolor(ANON_COLOR.0, ANON_COLOR.1, ANON_COLOR.2));
                }
            } else {
                copy_file(&source_path, &target_path)?;
                if progress_bar.is_none() {
                    println!("{} {} -> {}", "📋".truecolor(RANA_COLOR.0, RANA_COLOR.1, RANA_COLOR.2), source_path.display().to_string().white().dimmed(), target_name.truecolor(ANON_COLOR.0, ANON_COLOR.1, ANON_COLOR.2));
                }
            }
            
            if let Some(ref pb) = progress_bar {
                pb.inc(1);
            }
        }
    }

    if let Some(pb) = progress_bar {
        pb.finish_with_message("完成 | Completed".truecolor(RANA_COLOR.0, RANA_COLOR.1, RANA_COLOR.2).to_string());
    }

    if args.preview {
        println!();
        println!("{}", "💡 使用 --preview 查看操作预览，去掉该参数执行实际操作".truecolor(SOYO_COLOR.0, SOYO_COLOR.1, SOYO_COLOR.2));
        println!("{}", "   Use --preview to see operation preview, remove it to execute".truecolor(SOYO_COLOR.0, SOYO_COLOR.1, SOYO_COLOR.2).dimmed());
    } else {
        println!();
        println!("{}", "🎉 扁平化完成！就像爱音一样平整～".truecolor(ANON_COLOR.0, ANON_COLOR.1, ANON_COLOR.2).bold());
        println!("{}", "   Flattening completed! Flat as Anon~".truecolor(ANON_COLOR.0, ANON_COLOR.1, ANON_COLOR.2).dimmed());
    }

    Ok(())
}

/// 递归收集指定目录下的所有文件
pub fn collect_files(input_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in WalkDir::new(input_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            files.push(entry.path().to_path_buf());
        }
    }
    Ok(files)
}

/// 解决文件名冲突，为重名文件添加父目录后缀
/// 返回 (源路径, 目标文件名) 的映射
pub fn resolve_name_conflicts(files: &[PathBuf]) -> Vec<(PathBuf, String)> {
    let mut name_counts: HashMap<String, Vec<PathBuf>> = HashMap::new();
    
    for file_path in files {
        let original_name = file_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        name_counts.entry(original_name).or_insert_with(Vec::new).push(file_path.clone());
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

/// 获取文件名主体部分（不包含扩展名）
pub fn get_file_stem(filename: &str) -> &str {
    if let Some(pos) = filename.rfind('.') {
        &filename[..pos]
    } else {
        filename
    }
}

/// 获取文件扩展名（不包含点号）
pub fn get_file_extension(filename: &str) -> String {
    if let Some(pos) = filename.rfind('.') {
        filename[pos + 1..].to_string()
    } else {
        String::new()
    }
}

/// 移动文件到目标位置（剪切操作）
pub fn move_file(source: &Path, target: &Path) -> Result<()> {
    if target.exists() {
        std::fs::remove_file(target)
            .with_context(|| format!("无法删除已存在的目标文件 | Failed to remove existing target file: {}", target.display()))?;
    }

    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("无法创建目标目录 | Failed to create target directory: {}", parent.display()))?;
    }

    std::fs::rename(source, target)
        .with_context(|| format!("移动文件失败 | Failed to move file: {} -> {}", source.display(), target.display()))?;

    Ok(())
}

/// 复制文件到目标位置（保留源文件）
pub fn copy_file(source: &Path, target: &Path) -> Result<()> {
    if target.exists() {
        std::fs::remove_file(target)
            .with_context(|| format!("无法删除已存在的目标文件 | Failed to remove existing target file: {}", target.display()))?;
    }

    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("无法创建目标目录 | Failed to create target directory: {}", parent.display()))?;
    }

    std::fs::copy(source, target)
        .with_context(|| format!("复制文件失败 | Failed to copy file: {} -> {}", source.display(), target.display()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    /// 创建测试用的临时目录结构
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
        fs::write(source_dir.join("images/screenshots/screen1.png"), "screenshot1").unwrap();
        fs::write(source_dir.join("images/screenshots/screen2.png"), "screenshot2").unwrap();
        fs::write(source_dir.join("duplicate/file1.txt"), "duplicate content").unwrap();

        (temp_dir, source_dir)
    }

    #[test]
    fn test_collect_files() {
        let (_temp_dir, source_dir) = create_test_structure();
        let files = collect_files(&source_dir).unwrap();
        assert_eq!(files.len(), 7);
        
        let file_names: Vec<String> = files
            .iter()
            .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
            .collect();
        
        assert!(file_names.contains(&"file1.txt".to_string()));
        assert_eq!(file_names.iter().filter(|&name| name == "file1.txt").count(), 2);
    }

    #[test]
    fn test_resolve_name_conflicts() {
        let (_temp_dir, source_dir) = create_test_structure();
        let files = collect_files(&source_dir).unwrap();
        let name_map = resolve_name_conflicts(&files);
        
        assert_eq!(name_map.len(), 7);
        
        let target_names: Vec<String> = name_map.iter().map(|(_, name)| name.clone()).collect();
        let file1_count = target_names.iter().filter(|name| name.starts_with("file1")).count();
        assert_eq!(file1_count, 2);
        assert!(target_names.contains(&"file1.txt".to_string()));
        
        let mut sorted_names = target_names.clone();
        sorted_names.sort();
        sorted_names.dedup();
        assert_eq!(sorted_names.len(), target_names.len());
    }

    #[test]
    fn test_get_file_stem_and_extension() {
        assert_eq!(get_file_stem("file.txt"), "file");
        assert_eq!(get_file_stem("no_extension"), "no_extension");
        assert_eq!(get_file_extension("file.txt"), "txt");
        assert_eq!(get_file_extension("no_extension"), "");
    }

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
}