use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter};
use std::net::{SocketAddr, ToSocketAddrs};
use serde::{Deserialize, Serialize};

/// Web的监听配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebConfig {
    ipv4_socket: String,
    ipv6_socket: String
}
impl Default for WebConfig {
    fn default() -> Self {
        Self {
            ipv4_socket: String::from("0.0.0.0:80"),
            ipv6_socket: String::from("")
        }
    }
}
impl WebConfig {
    pub fn socket_addr(&self) -> io::Result<Vec<SocketAddr>> {
        //处理IPv4的SocketAddress
        let mut vec = Vec::new();
        vec.extend(self.ipv4_socket.to_socket_addrs()?);

        //如果IPv6不为空，那么用户一定配置了IPv6监听
        if !self.ipv6_socket.is_empty() {
            vec.extend(self.ipv6_socket.to_socket_addrs()?)
        }
        Ok(vec)
    }
}

/// 服务器运行所需要的Config配置文件，使用json，它将会决定服务器性能等因素
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub eula:bool,
    pub web_config: WebConfig
}
//这些值都可以被Default，不需要手动实现Default
/*
impl Default for Config {
    fn default() -> Self {
        Self {
            eula:false,
            web_config: WebConfig::default()
        }
    }
}
*/
impl Config {
    /// 读取默认目录的Config文件，如果出现文件不存在、不可读的情况，将会返回Err(e)
    pub fn read() -> io::Result<Config> {
        let mut reader = BufReader::new(File::open(".\\config.json")?);
        let config = serde_json::from_reader(&mut reader)?;
        Ok(config)
    }
    
    /// 创建默认的Config文件
    pub fn create_default_config() -> io::Result<()> {
        let default = Self::default();
        let mut writer = BufWriter::new(File::create(".\\config.json")?);
        serde_json::to_writer_pretty(&mut writer, &default)?;
        Ok(())
    }
}