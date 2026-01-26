//! # anon-flatten
//!
//! 一个简单的文件目录扁平化工具，让复杂的嵌套文件夹结构变得和爱音一样平。
//!
//! ## 功能特性
//!
//! - 递归遍历源文件夹
//! - 将所有嵌套文件扁平化到目标目录
//! - 智能处理重名文件冲突
//! - 支持复制或移动模式
//! - 预览模式查看操作结果
//!
//! ## 使用示例
//!
//! ```rust,no_run
//! use anon_flatten::flatten::{FlattenConfig, execute_flatten};
//! use std::path::PathBuf;
//!
//! let config = FlattenConfig {
//!     input: PathBuf::from("./source"),
//!     output: PathBuf::from("./target"),
//!     preview: false,
//!     cut: false,
//!     exclude_extensions: vec![],
//! };
//!
//! config.validate().unwrap();
//! let count = execute_flatten(&config, None::<fn(&str, usize, usize)>).unwrap();
//! println!("已处理 {} 个文件", count);
//! ```

pub mod cli;
pub mod colors;
pub mod conflict;
pub mod error;
pub mod file_ops;
pub mod flatten;

pub use error::{FlattenError, Result};
pub use flatten::{FlattenConfig, collect_files, execute_flatten, preview_operations};
