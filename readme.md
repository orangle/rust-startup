# Rust 

## 环境安装

### M1下安装

前提: 
```
xcode-select --install
```

安装
```
brew install rustup-init

rustup-init

source $HOME/.cargo/env
```

查看
```
# liuzhizhi @ liuzhizhideMacBook-Pro in ~/mygit [20:42:48] C:127
$ rustc --version
rustc 1.62.0 (a8314ef7d 2022-06-27)

# liuzhizhi @ liuzhizhideMacBook-Pro in ~/mygit [20:42:54]
$ cargo --version
cargo 1.62.0 (a748cf5a3 2022-06-08)
```

设置国内的cargo源

vim $HOME/.cargo/config

```
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"

replace-with = 'tuna'
[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

#replace-with = 'ustc'
#[source.ustc]
#registry = "git://mirrors.ustc.edu.cn/crates.io-index"

[net]
git-fetch-with-cli = true
```

