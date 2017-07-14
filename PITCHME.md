### こんにちは！

Rust lang...

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


### サンプルコード

``` rs
fn main() {
    println!("hello, world");
}
```
