# run_spike.sh hoge.elf hoge.log
set -euo pipefail

# .elfを受け取って走らす
spike --pc=0x80000000 --isa=rv32imf -m0x70000000:0x08000000,0x80000000:0x08000000 --log-commits "$1" 2> "$2"

# logを整形する
$(dirname $0)/convert_log_spike.sh "$2"