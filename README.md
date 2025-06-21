# anon-flatten 🎸

<div align="center">
  <img src="assets/image.png" alt="Anon Chihaya" width="300"/>
  
  <em style="color: #FF8899; font-weight: bold;">一个简单的文件目录扁平化工具，让复杂的嵌套文件夹结构变得和爱音一样平。</em>
  
  [![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
  [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
</div>

## ✨ 功能特性

- 🔍 **递归遍历** - 深入探索源文件夹的每一个角落
- 📁 **扁平化处理** - 将所有嵌套文件整理到单一目录
- 🛡️ **冲突处理** - 智能处理重名文件，避免覆盖
- ⚡ **高效安全** - 基于 Rust 构建，安全且高性能
- 🎯 **简单易用** - 一条命令搞定所有操作

## 🚀 快速开始

### 安装

```bash
# 从源码构建
git clone https://github.com/mygo-studio/anon-flatten.git
cd anon-flatten
cargo build --release
```

### 使用方法

```bash
# 基本用法
anon-flatten -i <源文件夹> -o <目标文件夹>

# 示例
anon-flatten -i ./messy_folders -o ./organized_flat
```

## 📖 使用示例

假设你有这样的文件结构：

```
messy_folders/
├── docs/
│   ├── report.pdf
│   └── notes/
│       └── meeting.txt
├── images/
│   ├── photo1.jpg
│   └── screenshots/
│       └── screen.png
└── code/
    └── main.rs
```

运行 `anon-flatten -i ./messy_folders -o ./flat_output` 后：

```
flat_output/
├── report.pdf
├── meeting.txt
├── photo1.jpg
├── screen.png
└── main.rs
```

就像千早爱音一样，简单直接，一马平川！🎸

## 🛠️ 开发

### 依赖项

```toml
[package]
name = "anon-flatten"
version = "0.1.0"
edition = "2024"

[dependencies]
anyhow = "1.0.98"
clap = { version = "4.5.40", features = ["derive"] }
colored = "3.0.0"
fs_extra = "1.3.0"
indicatif = "0.17.11"
walkdir = "2.5.0"
```

### 构建

```bash
# 开发构建
cargo build

# 发布构建
cargo build --release

# 运行测试
cargo test
```

## 📝 TODO

- [x] 添加进度条显示
- [x] 添加预览模式（不实际移动文件）
- [ ] 支持文件过滤（按扩展名/大小）
- [ ] 支持软链接处理
- [ ] 添加配置文件支持

## 📜 许可证

本项目采用 MIT 许可证。详见 [LICENSE](LICENSE) 文件。

## 🎨 致谢

- 角色设计来自 [BanG Dream!](https://bang-dream.com/) 的千早爱音
- 原图作者：[Pixiv - 130108237](https://www.pixiv.net/artworks/130108237)

---

<div align="center">
  <i>"既然要做，就要当最引人注目的扁平化工具！" —— 千早爱音</i>
</div>