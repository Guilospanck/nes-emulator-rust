; The stack in the 6502 works exactly like other stacks.
; It'll push to the top of the stack and then pop it (pull it).
;
; The stack lives in memory between 0x0100 and 0x01ff.
; The Stack Pointer (SP) points to where the stack is currently at.
; It begins at $ff which is the address 0x01ff.
;
; Two of the stack instrutions are PHA (Push Accumulator) and PLA (Pull Accumulator).

  LDY #$01              ; Loads the value 0x00 to the Y Register. It holds the position of our pixel, for our example.
  LDX #$01              ; Loads the value 0x00 to the X Register. It holds the color of the pixel, for our example.

forward_draw:
  TXA                   ; Loads the value of the X Register into the Accumulator.
  PHA                   ; Pushes the value inside the Accumulator onto the Stack.
  STA $0200, Y          ; Draws the pixel at the position 0x0200 + Y with the color that is contained inside the Accumulator.
  INX                   ; Increments the X Register.
  INY                   ; Increments the Y Register.
  CPY #$10              ; Compares the value inside the Y register with the value 0x10.
  BNE forward_draw      ; If Y is not 0x10, do the loop again

backward_draw:
  PLA                   ; Pulls the value from the Stack into the Accumulator.
  STA $0300, Y          ; Draws the pixel at the position 0x0300 + Y with the color that's inside the Accumulator.
  INY                   ; Increments the Y Register
  CPY #$20              ; Verifies if the value in the Y Register is 0x20
  BNE backward_draw     ; If it's not, do the loop again.