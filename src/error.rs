use thiserror::Error;

#[derive(Error, Debug)]
pub enum FlattenError {
    #[error("源文件夹不存在 | Source directory does not exist: {0}")]
    SourceNotFound(String),

    #[error("源路径不是文件夹 | Source path is not a directory: {0}")]
    SourceNotDirectory(String),

    #[error("目标文件夹不能在源文件夹内部 | Target directory cannot be inside source directory")]
    TargetInsideSource,

    #[error("无法创建目标文件夹 | Failed to create target directory: {0}")]
    CreateTargetDirFailed(String),

    #[error("无法删除已存在的目标文件 | Failed to remove existing target file: {0}")]
    RemoveExistingFileFailed(String),

    #[error("移动文件失败 | Failed to move file: {0} -> {1}")]
    MoveFileFailed(String, String),

    #[error("复制文件失败 | Failed to copy file: {0} -> {1}")]
    CopyFileFailed(String, String),

    #[error("IO 错误 | IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, FlattenError>;
