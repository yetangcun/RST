# RST
rust applications

# 安装rust
rustup install stable

# 查看rust版本
rustc --version

# 更新rust的版本
rustup update

# 创建Cargo.toml
cargo init

# 创建应用程序 也就是创建二进制Crate
cargo new bytedemo

# 创建库Crate
cargo new --lib libdemo

# 编译打包
cargo build --release

# 创建应用程序 也就是创建二进制Crate
cargo new bytedemo

# 创建库Crate
cargo new --lib libdemo

# 编译打包
cargo build --release

# Debug 模式运行
cargo run

# Release 模式运行
cargo run --release

# 添加linux目标平台 步骤1
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-linux-gnu-gcc

# windows平台链接
stable-x86_64-pc-windows-msvc
stable-x86_64-pc-windows-gnu
linux: rustup default stable-x86_64-pc-windows-gnu 或 windows: rustup default stable-x86_64-pc-windows-msvc  # 设置默认

# 编译linux 目标平台的二进制Crate 步骤2
cargo build --target=x86_64-unknown-linux-gnu --release

# 创建本地分支
git branch rstbase 或 git checkout -b rstbase

# 推送本地分支到远程
git push origin rstbase 或 git push -u origin feature-branch

# 删除本地分支
git branch -d rstbase 

# 删除远程分支
git push origin --delete rstbase

# 查看所有远程分支
git branch -a 或 git branch -r

# https://docs.rs/utoipa/4.2.3/utoipa/index.html swagger ui utoipa api文档生成
# https://crates.io/

# swagger ui api文档生成 网页地址：http://localhost:8000/swagger-ui/  http://127.0.0.1:8080/swagger-ui/

[dependencies]
mysql = "*"
clickhouse_rs = "*"

# Copilot 开启提问快捷键：问答：Ctrl+I， 聊天：Ctrl+Alt+I

# 通义灵码快捷键：AI程序员：Ctrl+Shift+I，智能问答：Ctrl+Shift+L

# Cursor 常用快捷键：Ctrl+L打开聊天界面  Ctrl+K调出咨询框
