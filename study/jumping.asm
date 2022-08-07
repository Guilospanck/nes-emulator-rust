; Jumping works like branching with two main differences:
; - They are not conditioned executed (they don't need a condition in order to go to another place)
; - They take an absolute address, meaning that it'll take more bytes in the assembled code, but it'll
; allow to go to positions greated than 1 byte offset.
;
; We have JMP, which will only jump to another part of our code and never return.
; And we also have JSR (Jump to Subroutine) and RTS (Return from Subroutine), that works like calling and returning from a function.

  JSR init                    ; Jump To Subroutine (JSR) called init
  JSR loop                    ; Jump To Subroutine (JSR) called loop
  JSR end                     ; Jump To Subroutine (JSR) called end

init:
  LDX #$00                    ; Loads 0x00 value to the X register
  LDY #$00                    ; Loads 0x00 value to the Y register
  RTS                         ; Return from Subroutine (RTS): Returns from where it was called

loop:
  TXA                         ; Copies the value from X to the Accumulator
  STA $0200, Y                ; Writes to the address 0x0200 + Y the value that's inside the Accumulator
  INX                         ; Increments the value inside the X Register by 1
  INY                         ; Increments the value inside the Y Register by 1
  CPX #$05                    ; Compare to X register (CPX): verifies if value in the X Register is 0x05
  BNE loop                    ; If it is not, then get back to the start of the loop
  RTS                         ; Return from Subroutine (RTS): Returns from where it was called

end:
  LDA #$02                    ; Loads the Accumulator with the value 0x02
  STA $05ff                   ; Writes the value that exists inside the Accumulator to the address 0x05ff
  JMP break_program           ; Jumps to the break_program branching

break_program:
  BRK
