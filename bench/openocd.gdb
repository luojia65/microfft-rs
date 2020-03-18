target remote :3333
set print asm-demangle on
set print pretty on
monitor arm semihosting enable
monitor tpiu config internal itm.txt uart off 72000000
monitor itm port 0 on
load
continue
