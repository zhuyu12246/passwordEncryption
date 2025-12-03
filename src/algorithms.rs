use crate::errors::PasswordError;

/// 密码哈希算法的 trait 定义
/// 所有密码哈希实现都需要实现这个 trait
pub trait PasswordHasherImpl {
    /// 对密码进行哈希处理
    /// 
    /// # 参数
    /// * `password` - 需要哈希的原始密码
    /// * `salt` - 盐值，用于增加哈希的随机性
    /// 
    /// # 返回值
    /// 返回哈希后的字符串结果或错误信息
    fn hash_password(&self, password: &str, salt: &str) -> Result<String, PasswordError>;

    /// 验证密码与哈希值是否匹配
    /// 
    /// # 参数
    /// * `password` - 待验证的原始密码
    /// * `hashed` - 已存储的哈希值
    /// 
    /// # 返回值
    /// 返回布尔值表示密码是否匹配，或者返回错误信息
    fn verify_password(&self, password: &str, hashed: &str) -> Result<bool, PasswordError>;
}