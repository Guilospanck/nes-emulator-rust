; 6502 accepts many differents ways of addressing (13, if I'm not mistaken)

; =========================================================================================================================================================
; Absolute: as the name suggests, it describes the whole 'path' of the address
STA $c000         ; Stores the value of the Accumulator into memory at the address 0xc000

; =========================================================================================================================================================
; Zero page: this is a better way of addressing because it takes less memory allocation, but not all functions can use it
STA $c0

; =========================================================================================================================================================
; Zero page, x: this will address the zero page address + the value of the X register 
LDX #$05          ; Load X Register (LDX)
STA $c0, x        ; Store Accumulator (STA): Will store the value of acc into memory at the address 0xc0 + 0x05 = 0xc5

; =========================================================================================================================================================
; zero page, y: works the same as the above, but only for LDX and STX
LDY #$02          ; Load Y Register (LDY)
STX $d0, y        ; Store X Register (STX): Will store the value of X Register into memory at the address 0xd0 + 0x02 = 0xd2

; =========================================================================================================================================================
; absolute, x: works the same as zero page,x but with an absolute address
LDX #$05
STA $c000, x      ; Store Accumulator (STA): Will store the value of acc into memory at the address 0xc000 + 0x05 = 0xc005

; =========================================================================================================================================================
; absolute, y: works the same as zero page, y , but, unlikely the zero page version, it doesn't work with STX, but can be used with LDA and STA
LDY #$03
STA $d000, y      ; Store Accumulator (STA): Will store the value of acc into memory at the address 0xd000 + 0x03 = 0xd003

; =========================================================================================================================================================
; Immediate: is the one that values are used, not memory addresses
LDX #$01          ; Load X Register (LDX): Loads immediately the VALUE 0x01 to the X Register

; =========================================================================================================================================================
; Relative: this is used for branching instructions
  LDA #$01        ; loads the value 0x01 to the accumulator
  CMP #$02        ; compare the value inside the accumulator with the value 0x02
  BNE notequal    ; Branch if Not Equal (BNE): if the values are not equal, will branch to the label "notequal"
  STA $22         ; if they are equal, stores the value of the Accumulator into memory at the address 0x22
notequal:
  BRK             ; Breaks the code execution

;; If you hexdump the code above, you will see
;; a9 01 c9 02 d0 02 85 22 00
;; a9 and c9 are the LDA and CMP opcodes respectively, and the 01 and 02 after them are the values 0x01 and 0x02
;; d0 is the opcode for the BNE instruction. You can see that after it we have 0x02. Where does that come from?
;; It's telling us to jump over the next 2 bytes (85: STA and 22: 0x22) which is exactly what our label wants.
;; Now, if we change the code above a little bit to 
STA $2200         ; using absolute addressing
;; we'll have the folllowing hexdumps
;; a9 01 c9 02 d0 03 8d 00 22 00
;; Which tells us some things:
;; - the opcode for the STA changed from 85 to 8d because it is now using a different addressing
;; - the argument of the BNE (d0) is now 03, because we added one more byte to our address at STA. This is why using zero page when possible is
;;   better because the assembled code will be smaller.
;; - note that the byte representation here is Little Endian, therefore $2200 => 0x00 0x22

; =========================================================================================================================================================
; Implicit: some addressing are done implictly, like Incremental and Decremental ones
INX             ; Increments in 1 the value inside the X Register           
DEX             ; Decrements in 1 the value inside the X Register
INY             ; Increments in 1 the value inside the Y Register
DEY             ; Decrements in 1 the value inside the Y Register

; =========================================================================================================================================================
; Indirect: uses an absolute address to look up another address
LDA #$01
STA $f1         ; 0x00f1
LDA #$04
STA $f2         ; 0x00f2
JMP ($00f1)     ; deferences to 0x0401

;; When we do the above instructions, we get the following in the memory addresses:
;;
;; 00f0: 00 01 04 00 00 00 ...
;; 0100: 00 00 00 00 00 00 ...
;; ...
;;
;; and then we are indirectly getting an address on the Jump instruction (JMP) in the
;; absolute address starting at 0x00f1
;; at 0x00f1 we have the value 0x01
;; the next address (0x00f2) we have the value 0x04
;; As we use the Little Endian representation, the address we're going to jump at will be
;; 0x0401


; =========================================================================================================================================================
; Indexed Indirect: ($c0, x) => it's like a sum of the above with the zero page
LDX #$01

LDA #$05
STA $01       ; Address 0x01 has the value 0x05

LDA #$07
STA $02       ; Address 0x02 has the value 0x07

LDY #$0a
STY $0705     ; address 0x0705 has the value 0x0A

LDA ($00, X)  ; here we're going to load the accumulator with the value that exists at the deferenced address formed by ($00, X)
              ; - first we get the zero page out of it:
              ;     $00 + x => $00 + $01 = $01
              ; - now we get the address that begins at the address 0x01 (indirect addressing):
              ;     0000: 00 05 07 00 00 ...
              ;     0100: 00 00 00 00 00 ...
              ;     ...
              ;
              ;     the address is formed by (little endian): 0x0705
              ; - load to the accumulator the value that exists at the address 0x0705, which is 0x0A

; Final result: the accumulator will have a value of 0x0A

; =========================================================================================================================================================
; Indirect indexed ($c0), y => it's close to the above, but different. Instead of adding the x (or y) value to the address before dereferencing it,
;                              we deference the address (indirect) and then we add the value of the register to it and then look for the value at this
;;                             new formed address.
LDY #$01

LDA #$03
STA $02

LDA #$08
STA $03

LDX #$0B
STX $0804

LDA ($02), Y  ; here we'll load the accumulator with the value that exists at some address formed by ($02), Y
              ; here's how to to it:
              ; - Look at first 2 bytes of the memory address, initiating by the address 0x02:
              ;   0000: 00 00 03 08 00 00 00 ...
              ;   0100: 00 00 00 00 00 00 00 ...
              ;   ...
              ;
              ; - Get the address from that (little endian): 0x0803
              ; 
              ; - Add to that address the value that exists in the Y register:
              ;   Value in the Y Register = 0x01
              ;   Address: 0x0803
              ;   Sum of them: 0x0803 + 0x0001 = 0x0804
              ;
              ; - The new value of the accumulator will be the same value that exists at this addres 0x0804, which is 0x0B
;              
; Final Result: accumulator will have the value of 0x0B
