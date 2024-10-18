# 这是什么
这是一个 Rust Embedded 示例

仅适用于 `STM32F103C8T6` MCU

LED 应位于PB12引脚, 配置 8MHz 外部晶振。

# 环境搭建

1. 安装Rust, 参见[这里](https://www.rust-lang.org/zh-CN/tools/install)
2. 添加工具链
```
rustup target add thumbv7m-none-eabi
```
3. 安装 `probe-rs` 工具集以烧录
```
cargo binstall probe-rs-tools
```
或通过[其他方式](https://probe.rs/docs/getting-started/installation/)安装

# 运行
将板子与下载器连接，之后运行:
```
cargo run --release
```
