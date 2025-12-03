use thiserror::Error;

/// 密码处理相关的错误类型
#[derive(Error, Debug)]
pub enum PasswordError {
    /// 哈希计算错误
    #[error("Hash error: {0}")]
    HashError(String),
    /// 密码验证错误
    #[error("Verify error: {0}")]
    VerifyError(String),
}