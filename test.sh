cd $(dirname $0)

mkdir -p build

INPUT=$1
BASE=$(basename "$INPUT" .s)
MY_BIN="build/${BASE}_my.bin"
REF_O="build/${BASE}_ref.o"
REF_BIN="build/${BASE}_ref.bin"
MY_TXT="build/${BASE}_my.txt"
REF_TXT="build/${BASE}_ref.txt"

cargo run --quiet -- --input "$INPUT" --output "$MY_BIN"
riscv64-unknown-elf-as -march=rv32imf -o "$REF_O" "$INPUT"
riscv64-unknown-elf-objcopy -O binary "$REF_O" "$REF_BIN"

xxd "$MY_BIN" > "$MY_TXT"
xxd "$REF_BIN" > "$REF_TXT"

if git diff --no-index --color=always "$REF_TXT" "$MY_TXT" > "build/${BASE}.diff"; then
    printf "\033[32mpassed\033[0m\n"
else
    printf "\033[31mfailed\033[0m\n"
    cat "build/${BASE}.diff"
fi