//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

use std::env;
use std::time::SystemTime;

fn main() {
    // 获取当前时间戳
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // 设置环境变量 TEST_FOO
    println!("cargo:rerun-if-changed=build.rs"); // 监测 build.rs 文件变化
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 启用 "pass" 特性
    println!("cargo:rustc-cfg=feature=\"pass\"");
}

