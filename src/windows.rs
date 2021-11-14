//! 执行windows平台命令 Execute windows platform commands
use std::time::SystemTime;
use std::{collections::HashMap, env, ffi::OsString};

use std::fs;
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::{Command, Stdio};
use std::str;

pub enum Vars {
    CargoHome(OsString),
    RustupDistServer(OsString),
    RustupUpdateRoot(OsString),
}

impl From<OsString> for Vars {
    fn from(u: OsString) -> Self {
        let cargo_home = OsString::from("CARGO_HOME");
        let rustup_dist_server = OsString::from("RUSTUP_DIST_SERVER");
        let rustup_update_root = OsString::from("RUSTUP_UPDATE_ROOT");
        if u == cargo_home {
            Vars::CargoHome(OsString::from("CARGO_HOME"))
        } else if u == rustup_dist_server {
            Vars::RustupDistServer(OsString::from("RUSTUP_DIST_SERVER"))
        } else if u == rustup_update_root {
            Vars::RustupUpdateRoot(OsString::from("RUSTUP_UPDATE_ROOT"))
        } else {
            unimplemented!()
        }
    }
}

pub fn shells(args: &str) {
    let _output = Command::new("cmd")
        .creation_flags(0x08000000)
        .arg("/c")
        .arg(args)
        .stdout(Stdio::piped())
        .output()
        .expect("cmd exec error!");
    // println!("cmd: {}", String::from_utf8_lossy(&output.stdout));
}

pub fn backup_rustup() {
    let back = |key: &OsString, value: OsString| -> () {
        let mut bak = key.clone();
        bak.push("_BAK");
        let k = bak.to_str();
        if k != None {
            let ks = k.unwrap();
            // setx /m name "value"
            let mut args = String::from("setx ");
            args.push_str(ks);
            args.push_str(" ");
            args.push_str(value.to_str().unwrap());
            // println!("args: {:?}", args);
            shells(&args);
            println!("{} 备份完成", ks);
        }
    };

    // println!("{:?}", args);

    // 环境变量中，字符并不一定是Unicode标准的，这样就会报错,可以使用vars_os
    let envs = env::vars_os();

    let mut map: HashMap<OsString, OsString> = HashMap::new();
    for (key, value) in envs {
        if key == "CARGO_HOME" || key == "RUSTUP_DIST_SERVER" || key == "RUSTUP_UPDATE_ROOT" {
            map.insert(key, value);
        }
    }
    for (key, value) in map {
        // println!("{:?} {:?}", key, value);
        match Vars::from(key) {
            Vars::CargoHome(key) => {
                // println!("{:?} {:?}", key, value);
                back(&key, value);
                // env::set_var(bak, value);
            }
            Vars::RustupDistServer(key) => {
                // println!("{:?} {:?}", key, value);
                back(&key, value);
            }
            Vars::RustupUpdateRoot(key) => {
                // println!("{:?} {:?}", key, value);
                back(&key, value);
            }
        }
    }
}

pub fn set_env(cn_server: &str, cn_server_root: &str) {
    let process = vec![
        ("RUSTUP_UPDATE_ROOT", cn_server_root),
        ("RUSTUP_DIST_SERVER", cn_server),
    ];

    for (key, value) in process {
        let mut args = String::from("setx ");
        args.push_str(key);
        args.push_str(" ");
        args.push_str(value);
        // println!("args: {:?}", args);
        shells(&args);
    }
    println!("\nrustup CN镜像设置完成：");
    let envs = env::vars_os();

    for (key, value) in envs {
        if key == "CARGO_HOME" || key == "RUSTUP_DIST_SERVER" || key == "RUSTUP_UPDATE_ROOT" {
            println!("{:?} {:?}", key, value);
        }
    }
}

pub fn back_crates(rep_path: &str) {
    let res = Command::new("cmd")
        .creation_flags(0x08000000)
        .arg("/c")
        .arg("echo %USERPROFILE%")
        // .arg("echo 哈哈") // 失败，windows默认中文为gbk非utf-8，转成utf-8会报错，路径最好不要有中文
        .stdout(Stdio::piped())
        .output()
        .expect("cmd exec error!");
    // 读取本地配置文件
    let rsproxy = fs::read(rep_path).unwrap();
    let file = str::from_utf8(&rsproxy).unwrap();
    // println!("{:?}", file);

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let mut user_path = String::from_utf8(res.stdout).unwrap();
    user_path = user_path.replace("\r\n", "");
    // println!("user_path:{:?}", user_path);
    let mut full_path = user_path.clone();
    full_path.push_str("\\.cargo");
    // println!("full_path:{:?}", full_path);
    let p = Path::new(&full_path);
    if p.is_dir() {
        // println!("{:?} 路径存在", p);
        let mut config_full_path = full_path.clone();
        config_full_path.push_str("\\config");
        let config_path = Path::new(&config_full_path);
        if config_path.is_file() {
            // user/config
            // println!("{:?} 路径存在", config_path);
            let mut config_full_path_bak = config_full_path.clone();
            config_full_path_bak.push_str("-");
            config_full_path_bak.push_str(&timestamp.to_string());
            config_full_path_bak.push_str(".bak");
            fs::copy(&config_full_path, &config_full_path_bak).unwrap(); // 备份
            println!("{:?} 备份成功", config_full_path_bak);
            // let mut before = fs::read(config_full_path).unwrap();
            // let mut cur = rsproxy.clone();
            // cur.append(&mut before);
            // println!("向前追加后: {:?}",str::from_utf8(&cur).unwrap());
            fs::write(&config_full_path, file).unwrap(); // 创建文件
        } else {
            fs::write(&config_full_path, file).unwrap(); // 创建文件
        }
        println!("{:?} 写入成功", config_full_path);
        println!("\ncrates CN镜像设置完成");
    } else {
        println!("没有{:?}目录，您好像没有安装Rust，请到 https://www.rust-lang.org/learn/get-started 下载。", full_path);
    }
}
pub fn windows(cn_server: &str, cn_server_root: &str, rep_path: &str) {
    println!("开始设置rustup CN镜像\n");
    backup_rustup();
    set_env(cn_server, cn_server_root);
    println!("\n");
    println!("开始设置crates CN镜像\n");
    back_crates(rep_path);
}
