# anon-flatten 🎸

<div align="center">
  <img src="assets/image.png" alt="Anon Chihaya" width="300"/>
  
  <em style="color: #FF8899; font-weight: bold;">一个简单的文件目录扁平化工具，让复杂的嵌套文件夹结构变得和爱音一样平。</em>
  
  [![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
  [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
  [![Crates.io](https://img.shields.io/crates/v/anon-flatten.svg?style=for-the-badge)](https://crates.io/crates/anon-flatten)
</div>

## ✨ 功能特性

- 🔍 **递归遍历** - 深入探索源文件夹的每一个角落
- 📁 **扁平化处理** - 将所有嵌套文件整理到单一目录
- 🛡️ **冲突处理** - 智能处理重名文件，避免覆盖
- 🎯 **文件过滤** - 按扩展名排除不需要的文件
- 🎨 **彩色输出** - MyGO!!!!! 主题配色，赏心悦目
- 📊 **进度显示** - 实时显示处理进度
- 👀 **预览模式** - 先看后做，安全可靠
- ✂️ **双模式** - 支持复制或剪切操作
- ⚡ **高效安全** - 基于 Rust 构建，安全且高性能

## 🚀 快速开始

### 从 Crates.io 安装

```bash
cargo install anon-flatten
```

### 从源码构建

```bash
git clone https://github.com/mygo-studio/anon-flatten.git
cd anon-flatten
cargo build --release
```

### 使用方法

```bash
# 基本用法（复制模式）
anon-flatten -i <源文件夹> -o <目标文件夹>

# 预览模式（不实际操作）
anon-flatten -i <源文件夹> -o <目标文件夹> --preview

# 剪切模式（移动文件）
anon-flatten -i <源文件夹> -o <目标文件夹> --cut

# 排除特定扩展名的文件（推荐：逗号分隔）
anon-flatten -i <源文件夹> -o <目标文件夹> --exclude txt,log,tmp

# 排除特定扩展名的文件（也支持多次指定）
anon-flatten -i <源文件夹> -o <目标文件夹> -e txt -e log -e tmp

# 示例
anon-flatten -i ./messy_folders -o ./organized_flat
anon-flatten -i ./downloads -o ./flat_downloads --preview
anon-flatten -i ./temp -o ./backup --cut
anon-flatten -i ./photos -o ./all_photos -e raw,tmp
```

### 参数说明

| 参数       | 短选项 | 长选项      | 说明                                 |
| ---------- | ------ | ----------- | ------------------------------------ |
| 源文件夹   | `-i`   | `--input`   | 要扁平化的源目录路径                 |
| 目标文件夹 | `-o`   | `--output`  | 扁平化后的目标目录路径               |
| 预览模式   | `-p`   | `--preview` | 只预览操作，不实际执行               |
| 剪切模式   | `-x`   | `--cut`     | 移动文件而非复制                     |
| 排除扩展名 | `-e`   | `--exclude` | 排除指定扩展名（逗号分隔或多次使用） |

## 📖 使用示例

### 示例 1: 基本扁平化

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

运行：

```bash
anon-flatten -i ./messy_folders -o ./flat_output
```

结果：

```
flat_output/
├── report.pdf
├── meeting.txt
├── photo1.jpg
├── screen.png
└── main.rs
```

### 示例 2: 处理重名文件

如果有重名文件：

```
source/
├── dir1/
│   └── file.txt
└── dir2/
    └── file.txt
```

扁平化后会自动添加父目录后缀：

```
target/
├── file.txt          # 来自 dir1
└── file_dir2.txt     # 来自 dir2
```

### 示例 3: 排除特定文件

排除临时文件和日志：

```bash
# 推荐写法：逗号分隔
anon-flatten -i ./project -o ./clean_files -e tmp,log,cache

# 也支持多次指定
anon-flatten -i ./project -o ./clean_files -e tmp -e log -e cache
```

假设源目录：

```
project/
├── src/
│   ├── main.rs
│   └── lib.rs
├── target/
│   └── debug.log
└── cache/
    └── temp.tmp
```

扁平化后只包含非排除类型的文件：

```
clean_files/
├── main.rs
└── lib.rs
```

### 示例 4: 预览模式

先预览操作：

```bash
anon-flatten -i ./source -o ./target --preview
```

输出示例：

```
🎸 开始扁平化操作 | Starting flatten operation...
📂 源文件夹 | Source:  ./source
📁 目标文件夹 | Target: ./target
🔄 操作模式 | Operation: 复制 | Copy

👀 预览模式 | Preview mode - operations to be performed:
📋 找到 | Found: 7 个文件 | files

  复制 | Copy      source/file1.txt -> file1.txt
  复制 | Copy      source/docs/report.pdf -> report.pdf
  复制 | Copy      source/duplicate/file1.txt -> file1_duplicate.txt
  ...
```

## 🛠️ 开发

### 构建

```bash
# 开发构建
cargo build

# 发布构建
cargo build --release

# 运行测试
cargo test

# 运行集成测试
cargo test --test integration_test

# 生成文档
cargo doc --open
```

### 作为库使用

```rust
use anon_flatten::flatten::{FlattenConfig, execute_flatten};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = FlattenConfig {
        input: PathBuf::from("./source"),
        output: PathBuf::from("./target"),
        preview: false,
        cut: false,
        exclude_extensions: vec!["tmp".to_string(), "log".to_string()],
    };

    config.validate()?;
    
    let count = execute_flatten(&config, Some(|filename, current, total| {
        println!("处理中: {} ({}/{})", filename, current, total);
    }))?;

    println!("已处理 {} 个文件", count);
    Ok(())
}
```

## 🎨 主题颜色

本项目使用 MyGO!!!!! 乐队成员的代表色：

| 成员     | 颜色 | RGB       | 用途       |
| -------- | ---- | --------- | ---------- |
| 千早爱音 | 粉色 | `#FF8899` | 主要强调色 |
| 高松灯   | 蓝色 | `#77BBDD` | 标签和提示 |
| 长崎爽世 | 黄色 | `#FFDD88` | 路径显示   |
| 椎名立希 | 紫色 | `#7777AA` | 错误和移动 |
| 要乐奈   | 绿色 | `#77DD77` | 成功和复制 |

## 📝 TODO

- [x] 添加进度条显示
- [x] 添加预览模式
- [x] 模块化代码结构
- [x] 完善单元测试
- [x] 添加集成测试
- [x] 自定义错误类型
- [x] 支持文件过滤（按扩展名）
- [ ] 支持文件大小过滤
- [ ] 支持软链接处理
- [ ] 添加配置文件支持
- [ ] 支持多线程并行处理
- [ ] 添加日志输出选项

## 📜 许可证

本项目采用 MIT 许可证。详见 [LICENSE](LICENSE) 文件。

## 🎨 致谬

- 角色设计来自 [BanG Dream!](https://bang-dream.com/) 的千早爱音
- 原图作者：[Pixiv - 130108237](https://www.pixiv.net/artworks/130108237)
- 灵感来源于 MyGO!!!!! 乐队

---

<div align="center">
  <i>"既然要做，就要当最引人注目的扁平化工具！" —— 千早爱音</i>
  
  就像爱音一样，简单直接，一马平川！🎸
</div>