# assembler

# 使い方
```shell
cargo run --input test/label.s
```

# テスト
riscv-unknown-gccのアセンブラが吐き出したバイナリと比較。独自命令の分はチェックできない
```shell
./test.sh test/label.s
```

# マクロなど
- レジスタ: `ra` `sp` `fp`