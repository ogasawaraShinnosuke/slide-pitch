### こんにちは！

Rust lang...

超雑に紹介＆ハンズオン

---

### なに

- 並列かつマルチパラダイムのプログラミング言語
- Mozilla Foundationが開発している

- 同時に複数の処理を演算可能
   - Javaだと、最近では `ParallelStream` などがある。
   - 今はほとんど見ることがない `Thread` という懐かしいものもそれに該当する。
   
---

### いつでてきたか

- Rust 1.0が日本時間の2015年5月16日にリリース
    - ※ [Wikipedia](https://ja.wikipedia.org/wiki/Rust_(%E3%83%97%E3%83%AD%E3%82%B0%E3%83%A9%E3%83%9F%E3%83%B3%E3%82%B0%E8%A8%80%E8%AA%9E))

---

<canvas data-chart="radar">


    Month, 1月, 2月, 3月, 4月, 5月, 6月, 7月
    1980, 65, 59, 80, 81, 56, 55, 40
    2017, 28, 48, 40, 19, 86, 27, 90


</canvas>

---


### オブジェクト

Impl
Trait
Struct

※継承なんてそもそも使わない

---

### パッケージ管理は？

- Cargo というものがやる
      - Rustのビルドシステム
      - Rustのパッケージマネージャ
      - Rustのプロジェクト管理
      - Javaで言うと，GradleとかMavenみたいなものと考えるとよいかも
      
---

### ハンズオン

やってみる


---

### インストール

**only Mac OS**

``` sh
brew install rust
rustc -V
cargo -V
```

---

### プロジェクトつくる

``` sh
cd
mkdir -p workspace/rust
cd workspace/rust
cargo new howdy --bin
cd howdy
```

---

### Cargo.toml

``` toml
[package]
name = "howdy"
version = "0.1.0"
authors = ["sin <ogasawaraShinnosuke@users.noreply.github.com>"]

[dependencies]
```
dependenciesってとこに追加ライブラリをかいていく

---

### src/main.rs

``` rs
fn main() {
    println!("Hello, world!");
}
```

こちらがbinのプログラムとなる

---

### びるど

``` shell
$ cargo build
   Compiling howdy v0.1.0 (file:///Users/ogasawarasusumunosuke/workspace/rust/howdy)
    Finished dev [unoptimized + debuginfo] target(s) in 1.47 secs
```

まずは，ビルドしましょう．基本この作業で依存関係も解決してくれます

---

### 中身

``` shell
tree
.
├── Cargo.lock
├── Cargo.toml
├── src
│   └── main.rs
└── target
    └── debug
        ├── build
        ├── deps
        │   ├── howdy-fe9f25f571817acc
        │   └── howdy-fe9f25f571817acc.dSYM
        │       └── Contents
        │           ├── Info.plist
        │           └── Resources
        │               └── DWARF
        │                   └── howdy-fe9f25f571817acc
        ├── examples
        ├── howdy
        ├── howdy.d
        ├── incremental
        └── native

12 directories, 8 files
```

--- 

### うごかす

``` shell
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/howdy`
Hello, world!
```

さきほどのmain.rsが起動しました．target/debug直下にあるコンパイルされたものが動きます

--- 


--- 
