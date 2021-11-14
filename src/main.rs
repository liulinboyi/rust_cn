use std::env;
pub mod windows;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // println!("Hello, world!");
    let rep_path = "static/rsproxy_config";
    let cn_server = "https://rsproxy.cn";
    let cn_server_root = "https://rsproxy.cn/rustup";
    if cfg!(target_os = "windows") {
        // println!("Hello Windows");
        let _args: Vec<String> = env::args().collect();
        windows::windows(cn_server, cn_server_root,rep_path);
    } else if cfg!(target_os = "linux") {
        println!("Hello Linux");
    } else {
        println!("Unknown os");
    }

    Ok(())
}
