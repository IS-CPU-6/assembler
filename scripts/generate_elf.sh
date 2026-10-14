dir=$(dirname "$1")
base=$(basename "$1" .bin)
name="$dir/$base"

riscv64-unknown-elf-objcopy \
  -I binary \
  -O elf32-littleriscv \
  --rename-section .data=.text \
  $name.bin $name.o

riscv64-unknown-elf-ld -m elf32lriscv -T "$(dirname "$0")/link.ld" "$name.o" -o "$name.elf"