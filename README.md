## `src/bin`

本編のプログラムに直接は関係無いけど、ちょっと書いて試したいとき。

`src/bin` を作っておき、その中に`.rs`ファイルを置いて書いていく。

例えば:

`src/bin/foo.rs`
```
fn main() {
    println!("{}", "foo");
}
```

などとファイルを作っておくと、以下のようにコマンドを実行できる。

```
$ cargo run --bin foo
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/foo`
foo
```

参考: https://natsutan.hatenablog.com/entry/2018/12/24/191720