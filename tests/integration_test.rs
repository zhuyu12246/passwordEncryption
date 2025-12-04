//! 密码哈希库的集成测试

// 引入需要测试的模块
use passwordEncryption::{PasswordHasher, Argon2Impl};

/// 测试 Argon2 算法的哈希和验证功能
#[test]
fn test_argon2_hash_and_verify() {
    // 创建 Argon2 算法实例
    let hasher = PasswordHasher::new(Box::new(Argon2Impl::default()));
    let password = "test_password";

    // 测试哈希生成功能
    let hashed_result = hasher.hash_password(password);
    assert!(hashed_result.is_ok());
    
    let hashed = hashed_result.unwrap();
    
    // 测试正确密码验证功能
    let verify_result = hasher.verify_password(password, &hashed);
    assert!(verify_result.is_ok());
    assert!(verify_result.unwrap());
    
    // 测试错误密码验证功能
    let wrong_password_result = hasher.verify_password("wrong_password", &hashed);
    assert!(wrong_password_result.is_ok());
    assert!(!wrong_password_result.unwrap());
}

/// 测试空密码的处理
#[test]
fn test_empty_password() {
    // 创建 Argon2 算法实例
    let hasher = PasswordHasher::new(Box::new(Argon2Impl::default()));
    let password = "";

    // 测试空密码的哈希生成
    let hashed_result = hasher.hash_password(password);
    assert!(hashed_result.is_ok());
    
    let hashed = hashed_result.unwrap();
    
    // 测试空密码的验证
    let verify_result = hasher.verify_password(password, &hashed);
    assert!(verify_result.is_ok());
    assert!(verify_result.unwrap());
}