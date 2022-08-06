; Branching is a kind of loop

  LDX #$03              ; loads 0x03 to the register X

decrement_loop:
  DEX                   ; decrements the value in the register X
  CPX #$01              ; Compares X Register (CPX): compares the value in the register X to the value 0x01. If they are equal, the Z flag will be set to 1, otherwise 0.
  BNE decrement_loop    ; Branch Not Equal (BNE). If the two values are not equal, go back to the start of the decrement_loop

increment_loop:
  INX                   ; increments the value in the register X
  CPX #$05              ; compares the value of the register X with the value 0x05
  BEQ subtracts_acc     ; if its five, break
  CPX #$0A              ; Compares X Register (CPX): compares the value of the register X with the value 0x0A
  BNE increment_loop    ; if the values are not equal, go back to the increment loop. otherwise, continue

break_program:  
  BRK                   ; breaks the program 

subtracts_acc:
  LDA #$06              ; sets the value of the accumulator to 0x06
  SEC                   ; sets the carry flag in order to do the right subtraction
  BCS subtracts_loop    ; Branch on Carry Set (BCS): will branch when the carry flag is set, which, in this case, happens above with the SEC command

subtracts_loop:
  SBC #$01              ; Subtracts With Carry (SBC) subtracts the value from the accumulator
  BCC break_program     ; Branch on Carry Clear (BCC): it will break the program once the carry flag is clear, which, in this case
                        ; will happen when the counter changes from positive to negative. (0 - 1 = -1)
  JMP subtracts_loop    ; go back to the loop
