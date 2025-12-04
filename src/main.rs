//! 密码哈希库的示例程序
//! 展示如何使用该库进行密码哈希和验证操作

// 引入我们创建的密码哈希库
use passwordEncryption::{PasswordHasher, Argon2Impl};

fn main() {
    // 创建一个使用 Argon2 算法的密码哈希器实例
    let hasher = PasswordHasher::new(Box::new(Argon2Impl::default()));
    
    // 定义测试密码
    let password = "my_secure_password";
    
    // 对密码进行哈希处理
    match hasher.hash_password(password) {
        Ok(hashed) => {
            // 输出原始密码和哈希后的密码
            println!("Original password: {}", password);
            println!("Hashed password: {}", hashed);
            
            // 验证正确密码
            match hasher.verify_password(password, &hashed) {
                Ok(true) => println!("Password verification successful!"),
                Ok(false) => println!("Password verification failed!"),
                Err(e) => println!("Error verifying password: {:?}", e),
            }
            
            // 验证错误密码
            match hasher.verify_password("wrong_password", &hashed) {
                Ok(true) => println!("Password verification successful!"),
                Ok(false) => println!("Password verification failed as expected!"),
                Err(e) => println!("Error verifying password: {:?}", e),
            }
        },
        Err(e) => println!("Error hashing password: {:?}", e),
    }
}