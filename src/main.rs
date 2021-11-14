//! # Rust_CN
//!
//! > **为Rust设置国内镜像**
//!
//! - 设置`rustup`和`crates`镜像
//!
//! - 镜像地址用的是[RsProxy.cn](https://rsproxy.cn/)
//!
//! - 未安装Rust也可以用，会设置好rustup的国内镜像地址
//!
//! [下载](https://github.com/liulinboyi/rust_cn/releases/latest)
//! 解压后在命令行执行以下命令即可
//! ```
//! .\rust_cn.exe
//! ```
//!
//! ![结果](https://github.com/liulinboyi/rust_cn/raw/main/image/result.png)
use std::env;
pub mod windows;

/// main函数 main function
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // println!("Hello, world!");
    let rep_path = "static/rsproxy_config";
    let cn_server = "https://rsproxy.cn";
    let cn_server_root = "https://rsproxy.cn/rustup";
    if cfg!(target_os = "windows") {
        // println!("Hello Windows");
        let _args: Vec<String> = env::args().collect();
        windows::windows(cn_server, cn_server_root, rep_path);
    } else if cfg!(target_os = "linux") {
        println!("Hello Linux");
    } else {
        println!("Unknown os");
    }

    Ok(())
}
