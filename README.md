# assembler
[![Release](https://img.shields.io/badge/version-v0.0.5-blue)](https://docs.google.com/spreadsheets/d/1nd4VlMRf07WgoH0fAjvWLOeL6DX_4LZD6E22ocDrY7Y/edit?hl=ja&gid=584437860#gid=584437860)

# 使い方
```shell
cargo run --input test/label.s --output hoge.bin
```

# テスト
riscv-unknown-gccのアセンブラが吐き出したバイナリと比較。独自命令の分はチェックできない
```shell
./test.sh test/label.s
```

# マクロなど
- レジスタ: `ra` `sp` `fp`