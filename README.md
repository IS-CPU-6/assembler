# assembler
[![Release](https://img.shields.io/badge/ISA-1st_v0.0.6-blue)](https://docs.google.com/spreadsheets/d/1nd4VlMRf07WgoH0fAjvWLOeL6DX_4LZD6E22ocDrY7Y/edit?hl=ja&gid=584437860#gid=584437860)

仕様はほぼRV32IMF

# 未実装
- 各種疑似命令

# 使い方
- .binを出力する (現時点では平行移動可能なバイナリになっている)
```shell
cargo run -- --input test/fibonacci.s --output fibonacci.bin
```

- `spike-sim` で実行する場合はこのようにする。
- 入力されたアセンブリの末尾にspike-simulatorの終了処理を足してからアセンブルしているので、プログラムが末尾以外で終了しないようにすること。ebreakは必要ない。
```shell
mkdir -p build
cargo run -- --input test/fibonacci.s --output build/fibonacci.bin --spike-sim
./scripts/generate_elf.sh build/fibonacci.bin # .binから.elfを作る
./scripts/run_spike.sh build/fibonacci.elf build/fibonacci.log # .elfをspikeに渡す。トレースが出る
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
