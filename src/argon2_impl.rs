use crate::algorithms::PasswordHasherImpl;
use crate::errors::PasswordError;
use argon2::{
    Argon2, Params, PasswordHasher as _, PasswordVerifier as _,
    password_hash::{SaltString, PasswordHash as PH},
};
use rand::rngs::OsRng;

/// Argon2 算法实现
/// Argon2 是一种内存硬化的密码哈希函数，被选为 Password Hashing Competition 的获胜者
pub struct Argon2Impl {
    /// Argon2 算法参数
    pub params: Params,
}

/// 为 Argon2Impl 实现默认构造函数
impl Default for Argon2Impl {
    fn default() -> Self {
        Self { params: Params::default() }
    }
}

/// 为 Argon2Impl 实现 PasswordHasherImpl trait
impl PasswordHasherImpl for Argon2Impl {
    /// 使用 Argon2 算法对密码进行哈希处理
    /// 
    /// # 参数
    /// * `password` - 需要哈希的原始密码
    /// * `_salt` - 盐值（注意：此参数在此实现中未使用，因为 Argon2 会自动生成安全的盐值）
    /// 
    /// # 返回值
    /// 返回哈希后的字符串结果或错误信息
    fn hash_password(&self, password: &str, _salt: &str) -> Result<String, PasswordError> {
        // 创建默认的 Argon2 实例
        let argon2 = Argon2::default();
        
        // 生成加密安全的随机盐值
        let salt = SaltString::generate(&mut OsRng);
        
        // 使用 Argon2 算法对密码进行哈希处理
        match argon2.hash_password(password.as_bytes(), &salt) {
            Ok(pw_hash) => Ok(pw_hash.to_string()),
            Err(e) => Err(PasswordError::HashError(e.to_string())),
        }
    }

    /// 验证密码与哈希值是否匹配
    /// 
    /// # 参数
    /// * `password` - 待验证的原始密码
    /// * `hashed` - 已存储的哈希值
    /// 
    /// # 返回值
    /// 返回布尔值表示密码是否匹配，或者返回错误信息
    fn verify_password(&self, password: &str, hashed: &str) -> Result<bool, PasswordError> {
        // 解析已存储的哈希值
        let parsed = match PH::new(hashed) {
            Ok(p) => p,
            Err(e) => return Err(PasswordError::VerifyError(e.to_string())),
        };
        
        // 创建默认的 Argon2 实例
        let argon2 = Argon2::default();
        
        // 验证密码与哈希值是否匹配
        match argon2.verify_password(password.as_bytes(), &parsed) {
            // 如果验证成功，返回 true
            Ok(()) => Ok(true),
            // 如果密码不匹配，返回 false
            Err(argon2::password_hash::Error::Password) => Ok(false),
            // 如果发生其他错误，返回错误信息
            Err(e) => Err(PasswordError::VerifyError(e.to_string())),
        }
    }
}