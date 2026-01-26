use anon_flatten::cli::Args;
use anon_flatten::colors::*;
use anon_flatten::error::FlattenError;
use anon_flatten::flatten::{FlattenConfig, execute_flatten, preview_operations};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};

fn main() {
    if let Err(e) = run() {
        eprintln!(
            "{}",
            format!("❌ 错误 | Error: {}", e)
                .truecolor(TAKI_COLOR.0, TAKI_COLOR.1, TAKI_COLOR.2)
                .bold()
        );
        std::process::exit(1);
    }
}

fn run() -> Result<(), FlattenError> {
    let args = Args::parse_args();

    // 创建配置
    let config = FlattenConfig {
        input: args.input.clone(),
        output: args.output.clone(),
        preview: args.preview,
        cut: args.cut,
        exclude_extensions: args.exclude,
    };

    // 验证配置
    config.validate()?;

    // 打印操作信息
    print_header(&config);

    // 预览模式或执行模式
    if config.preview {
        run_preview_mode(&config)?;
    } else {
        run_execute_mode(&config)?;
    }

    Ok(())
}

/// 打印操作头部信息
fn print_header(config: &FlattenConfig) {
    println!(
        "{}",
        "🎸 开始扁平化操作 | Starting flatten operation..."
            .truecolor(ANON_COLOR.0, ANON_COLOR.1, ANON_COLOR.2)
            .bold()
    );
    println!(
        "{:<20} {}",
        "📂 源文件夹 | Source:".truecolor(TOMORI_COLOR.0, TOMORI_COLOR.1, TOMORI_COLOR.2),
        config
            .input
            .display()
            .to_string()
            .truecolor(SOYO_COLOR.0, SOYO_COLOR.1, SOYO_COLOR.2)
    );
    println!(
        "{:<20} {}",
        "📁 目标文件夹 | Target:".truecolor(TOMORI_COLOR.0, TOMORI_COLOR.1, TOMORI_COLOR.2),
        config
            .output
            .display()
            .to_string()
            .truecolor(SOYO_COLOR.0, SOYO_COLOR.1, SOYO_COLOR.2)
    );

    let operation = if config.cut {
        "移动 | Move"
            .truecolor(TAKI_COLOR.0, TAKI_COLOR.1, TAKI_COLOR.2)
            .bold()
    } else {
        "复制 | Copy"
            .truecolor(RANA_COLOR.0, RANA_COLOR.1, RANA_COLOR.2)
            .bold()
    };
    println!(
        "{:<20} {}",
        "🔄 操作模式 | Operation:".truecolor(TOMORI_COLOR.0, TOMORI_COLOR.1, TOMORI_COLOR.2),
        operation
    );

    if !config.exclude_extensions.is_empty() {
        println!(
            "{:<20} {}",
            "🚫 排除扩展名 | Exclude:".truecolor(TOMORI_COLOR.0, TOMORI_COLOR.1, TOMORI_COLOR.2),
            config.exclude_extensions.join(", ").truecolor(
                TAKI_COLOR.0,
                TAKI_COLOR.1,
                TAKI_COLOR.2
            )
        );
    }

    println!();
}

/// 运行预览模式
fn run_preview_mode(config: &FlattenConfig) -> Result<(), FlattenError> {
    println!(
        "{}",
        "👀 预览模式 | Preview mode - operations to be performed:"
            .truecolor(ANON_COLOR.0, ANON_COLOR.1, ANON_COLOR.2)
            .bold()
    );

    let operations = preview_operations(config)?;

    if operations.is_empty() {
        println!(
            "{}",
            "📭 源文件夹中没有找到任何文件 | No files found in source directory".truecolor(
                SOYO_COLOR.0,
                SOYO_COLOR.1,
                SOYO_COLOR.2
            )
        );
        return Ok(());
    }

    println!(
        "{:<20} {} {}",
        "📋 找到 | Found:".truecolor(TOMORI_COLOR.0, TOMORI_COLOR.1, TOMORI_COLOR.2),
        operations
            .len()
            .to_string()
            .truecolor(ANON_COLOR.0, ANON_COLOR.1, ANON_COLOR.2)
            .bold(),
        "个文件 | files".truecolor(TOMORI_COLOR.0, TOMORI_COLOR.1, TOMORI_COLOR.2)
    );
    println!();

    for (source_path, target_name) in operations {
        let op = if config.cut {
            "移动 | Move".truecolor(TAKI_COLOR.0, TAKI_COLOR.1, TAKI_COLOR.2)
        } else {
            "复制 | Copy".truecolor(RANA_COLOR.0, RANA_COLOR.1, RANA_COLOR.2)
        };
        println!(
            "  {:<15} {} -> {}",
            op,
            source_path.display().to_string().white().dimmed(),
            target_name.truecolor(ANON_COLOR.0, ANON_COLOR.1, ANON_COLOR.2)
        );
    }

    println!();
    println!(
        "{}",
        "💡 使用 --preview 查看操作预览，去掉该参数执行实际操作".truecolor(
            SOYO_COLOR.0,
            SOYO_COLOR.1,
            SOYO_COLOR.2
        )
    );
    println!(
        "{}",
        "   Use --preview to see operation preview, remove it to execute"
            .truecolor(SOYO_COLOR.0, SOYO_COLOR.1, SOYO_COLOR.2)
            .dimmed()
    );

    Ok(())
}

/// 运行执行模式
fn run_execute_mode(config: &FlattenConfig) -> Result<(), FlattenError> {
    // 创建进度条
    let progress_bar = ProgressBar::new(0);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("🎸 {spinner:.magenta} [{elapsed_precise}] [{bar:40.magenta/white}] {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("█▇▆▅▄▃▂▁  ")
    );

    // 定义进度回调
    let callback = |filename: &str, current: usize, total: usize| {
        if progress_bar.length().unwrap_or(0) == 0 {
            progress_bar.set_length(total as u64);
        }

        let display_name = if filename.len() > 30 {
            format!("{}...", &filename[..27])
        } else {
            filename.to_string()
        };

        progress_bar.set_message(display_name);
        progress_bar.set_position(current as u64);
    };

    // 执行扁平化
    let count = execute_flatten(config, Some(callback))?;

    progress_bar.finish_with_message(
        "完成 | Completed"
            .truecolor(RANA_COLOR.0, RANA_COLOR.1, RANA_COLOR.2)
            .to_string(),
    );

    if count == 0 {
        println!(
            "{}",
            "📭 源文件夹中没有找到任何文件 | No files found in source directory".truecolor(
                SOYO_COLOR.0,
                SOYO_COLOR.1,
                SOYO_COLOR.2
            )
        );
    } else {
        println!();
        println!(
            "{}",
            "🎉 扁平化完成！就像爱音一样平整～"
                .truecolor(ANON_COLOR.0, ANON_COLOR.1, ANON_COLOR.2)
                .bold()
        );
        println!(
            "{}",
            "   Flattening completed! Flat as Anon~"
                .truecolor(ANON_COLOR.0, ANON_COLOR.1, ANON_COLOR.2)
                .dimmed()
        );
    }

    Ok(())
}
