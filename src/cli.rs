use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "anon-flatten",
    about = "一个简单的文件目录扁平化工具，让复杂的嵌套文件夹结构变得和爱音一样平 | A simple file directory flattening tool",
    version,
    after_help = "\x1b[38;2;255;136;153m🎸 让你的文件夹像千早爱音一样，简单直接，一马平川！\x1b[0m"
)]
pub struct Args {
    /// 源文件夹路径 | Source directory path
    #[arg(short = 'i', long = "input")]
    pub input: PathBuf,

    /// 目标文件夹路径 | Target directory path
    #[arg(short = 'o', long = "output")]
    pub output: PathBuf,

    /// 预览模式 | Preview mode
    #[arg(short = 'p', long = "preview")]
    pub preview: bool,

    /// 剪切模式 | Cut mode
    #[arg(short = 'x', long = "cut")]
    pub cut: bool,

    /// 排除的文件扩展名（逗号分隔或多次指定）| Exclude file extensions (comma-separated or multiple times)
    ///
    /// Example: -e txt,log,tmp  or  -e txt -e log -e tmp
    #[arg(
        short = 'e',
        long = "exclude",
        value_name = "EXT",
        value_delimiter = ','
    )]
    pub exclude: Vec<String>,
}

impl Args {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
