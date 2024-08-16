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

[dependencies]
mysql = "*"
clickhouse_rs = "*"

