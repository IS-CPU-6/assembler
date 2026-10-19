set -euo pipefail
dir=$(dirname "$1")
base=$(basename "$1" .bin)
name="$dir/$base"
attr=$(dirname "$0")/attr

# source .bin -> .o
# riscv64-unknown-elf-objcopy -I binary -O elf32-littleriscv -B riscv:rv32imf --rename-section .data=.text $name.bin $name.o

# touch $attr.s
# printf '.section .text\n.global tohost\n.global fromhost\n.incbin "%s.bin"\n' "$name" > $attr.s

# attr .s -> .o
riscv64-unknown-elf-as -march=rv32imf -mabi=ilp32f $attr.s -o $attr.o

# .o -> .elf
riscv64-unknown-elf-ld -m elf32lriscv -T $(dirname "$0")/link.ld $attr.o -o $name.elf

riscv64-unknown-elf-objdump -s $name.elf


# rm -rf $attr.o