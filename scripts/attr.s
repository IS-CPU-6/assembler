.section .text
.global tohost
.global fromhost
lui  t0, 0x6          # t0 = 0x6000 (FS=Dirty)
csrw mstatus, t0
.incbin "./scripts/../build/add.bin"
