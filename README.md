# assembler
[![Release](https://img.shields.io/badge/ISA-1st_v0.0.6-blue)](https://docs.google.com/spreadsheets/d/1nd4VlMRf07WgoH0fAjvWLOeL6DX_4LZD6E22ocDrY7Y/edit?hl=ja&gid=584437860#gid=584437860)

仕様はほぼRV32IMF

# 使い方
```shell
cargo run -- --input test/label.s --output hoge.bin
```

# テスト
riscv-unknown-gccのアセンブラが吐き出したバイナリと比較。独自命令の分はチェックできない
```shell
./test.sh test/label.s
```

# マクロなど
- レジスタ: `ra` `sp` `fp`

# riscv-unknown-gccと異なる特殊仕様
- ジャンプ命令のoffsetに整数即値も入れられる
- 浮動小数点演算の[丸めモード](https://docs.riscv.org/reference/isa/v20260120/unpriv/f-st-ext.html)フィールド
    - 通常は無条件で`0b111`を出力
    - `fmin.s`, `fmax.s`の場合それぞれ`0b000`, `0b001`を出力
    - ISA 1st v0.0.6時点ではこのフィールドに何が入っていても動作は`0b000(RNE)`相当