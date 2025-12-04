//! 一个安全的密码哈希库
//! 
//! 这个库提供了安全的密码哈希和验证功能，使用 industry-standard 的 Argon2 算法。
//! 它可以防止彩虹表攻击、暴力破解和其他常见的密码攻击方式。

/// 密码哈希算法 trait 模块
mod algorithms;
/// Argon2 算法实现模块
mod argon2_impl;
/// 错误类型定义模块
mod errors;

// 公开接口导出
pub use algorithms::PasswordHasherImpl;
pub use argon2_impl::Argon2Impl;
pub use errors::PasswordError;

/// 密码哈希器主结构体
/// 提供统一的接口来使用不同的哈希算法
pub struct PasswordHasher {
    /// 实际使用的哈希算法实现
    algorithm: Box<dyn PasswordHasherImpl>,
}

impl PasswordHasher {
    /// 创建一个新的密码哈希器实例
    /// 
    /// # 参数
    /// * `algorithm` - 实现了 PasswordHasherImpl trait 的具体算法实现
    /// 
    /// # 返回值
    /// 返回一个新的 PasswordHasher 实例
    pub fn new(algorithm: Box<dyn PasswordHasherImpl>) -> Self {
        Self { algorithm }
    }

    /// 对密码进行哈希处理
    /// 
    /// # 参数
    /// * `password` - 需要哈希的原始密码
    /// * `salt` - 盐值
    /// 
    /// # 返回值
    /// 返回哈希后的字符串结果或错误信息
    pub fn hash_password(&self, password: &str) -> Result<String, PasswordError> {
        self.algorithm.hash_password(password)
    }

    /// 验证密码与哈希值是否匹配
    /// 
    /// # 参数
    /// * `password` - 待验证的原始密码
    /// * `hashed` - 已存储的哈希值
    /// 
    /// # 返回值
    /// 返回布尔值表示密码是否匹配，或者返回错误信息
    pub fn verify_password(&self, password: &str, hashed: &str) -> Result<bool, PasswordError> {
        self.algorithm.verify_password(password, hashed)
    }
}