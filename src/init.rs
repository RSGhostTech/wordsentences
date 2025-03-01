use std::process::exit;
use crate::utils::config::Config;

/// 加载配置文件
pub fn init_config() -> Config {
    Config::read().unwrap_or_else(|e| {
        println!("无法读取配置文件，尝试创建配置文件\r\n{}", e);
        
        if let Err(e) = Config::create_default_config() {
            println!("无法创建配置文件\r\n{}", e);
            exit(1);
        } else {
            println!("初始化配置文件创建完成，请进行配置");
            exit(0)
        }
    })
}

pub fn check_eula(config: &Config) {
    if !config.eula {
        println!("没有同意eula，请查看eula，同意以后才能使用程序");
        exit(1);
    }
}