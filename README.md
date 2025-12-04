# Password Encryption

[![license](https://badgen.net/badge/license/MIT/blue)](https://github.com/yourusername/passwordEncryption/blob/master/LICENSE)
[![rust](https://badgen.net/badge/language/Rust/red?icon=rust)](https://www.rust-lang.org/)

一个安全的密码哈希 Rust 库，使用 industry-standard 的 Argon2 算法来保护用户密码。

## 特性

- 使用 Argon2 算法，这是一种内存硬化的密码哈希函数，被选为 Password Hashing Competition 的获胜者
- 自动生成加密安全的盐值
- 提供简单易用的 API 接口
- 支持密码哈希和验证功能
- 具有良好的错误处理机制

## 依赖

- [argon2](https://crates.io/crates/argon2) - Argon2 密码哈希算法的 Rust 实现
- [rand](https://crates.io/crates/rand) - 用于生成加密安全的随机数
- [thiserror](https://crates.io/crates/thiserror) - 用于简化错误处理的宏

## 安装

在你的 `Cargo.toml` 文件中添加以下内容：

```toml
[dependencies]
passwordEncryption = "0.1"
```

## 使用方法

```rust
use passwordEncryption::{PasswordHasher, Argon2Impl};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建一个使用 Argon2 算法的密码哈希器实例
    let hasher = PasswordHasher::new(Box::new(Argon2Impl::default()));
    
    let password = "my_secure_password";
    
    // 对密码进行哈希处理
    let hashed = hasher.hash_password(password)?;
    println!("Original password: {}", password);
    println!("Hashed password: {}", hashed);
    
    // 验证正确密码
    assert!(hasher.verify_password(password, &hashed)?);
    
    // 验证错误密码
    assert!(!hasher.verify_password("wrong_password", &hashed)?);
    
    Ok(())
}
```

## API 文档

有关详细信息，请参阅 [API documentation](https://docs.rs/passwordEncryption).

## 许可证

本项目采用 MIT 许可证。详情请见 [LICENSE](LICENSE) 文件。