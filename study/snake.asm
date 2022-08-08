; This is a simple snake game built in assembly language 6502.

;; System variables
define system_last_key $ff ; gets the last pressed key

;; ASCII WASD Keys
define w_key $77 ; w key in ascii hex format
define a_key $61 ; a key in ascii hex format
define s_key $73 ; s key in ascii hex format
define d_key $64 ; d key in ascii hex format

;; Snake Properties
define snake_length_addr $00                     ; address 0x02 will have the data for the snake length
define snake_direction_addr $01                  ; address 0x03 will have the data for the snake direction

define snake_head_low_byte_addr $02              ; address 0x04 will have the data for the snake head low byte
define snake_head_high_byte_addr $03             ; address 0x05 will have the data for the snake head high byte
define snake_tail_low_byte_addr $04              ; address 0x06 will have the data for the snake tail low byte
define snake_tail_high_byte_addr $05             ; address 0x07 will have the data for the snake tail high byte

define snake_color 4                        ; defines the color of the snake (purple)
define snake_bg_color 0                     ; defines the bg color (black)
define snake_length 5                       ; initial snake length value

;; Possible snake directions
define snake_up 1
define snake_right 2
define snake_bottom 3
define snake_left 4

; Main
  JSR init_snake
  JSR loop

init_snake:
  LDA #snake_length ; snake_length
  STA snake_length_addr

  LDA #snake_bottom ; snake direction
  STA snake_direction_addr

  ; draws snake
  LDX snake_length_addr
  LDA #snake_color
  JSR draw_snake

  ; update snake head (0x0204)
  LDX #$04
  LDY #$02
  JSR update_snake_head  

  ; update snake tail (0x0200)
  LDX #$00
  LDY #$02
  JSR update_snake_tail

  RTS

draw_snake:
  DEX
  STA $0200, X
  CPX #$00
  BNE draw_snake

  RTS

update_snake_head:
  STX snake_head_low_byte_addr
  STY snake_head_high_byte_addr

  RTS

update_snake_tail:
  STX snake_tail_low_byte_addr
  STY snake_tail_high_byte_addr

  RTS

loop:
  JSR read_keys

  ; moves snake  
  JSR move_snake

  ; infinite loop
  LDA #$01
  CMP #$02
  BNE loop
  BRK

; Basically a Switch-case
read_keys:
  LDA system_last_key

  CMP #w_key
  BEQ move_up

  CMP #d_key
  BEQ move_right

  CMP #s_key
  BEQ move_bottom

  CMP #a_key
  BEQ move_left

  RTS

move_up:
  LDA #snake_up
  STA snake_direction_addr
  RTS

move_right:
  LDA #snake_right
  STA snake_direction_addr
  RTS

move_bottom:
  LDA #snake_bottom
  STA snake_direction_addr
  RTS

move_left:
  LDA #snake_left
  STA snake_direction_addr
  RTS

move_snake:
  ;;;;; Tail
  ; LDA #snake_bg_color               ; loads the background color (black) into the Accumulator
  ; LDY #$00                          
  ; STA (snake_tail_low_byte_addr), Y ; indirect indexed addressing

  ; ; updates new tail
  ; LDX snake_tail_low_byte_addr
  ; INX
  ; STX snake_tail_low_byte_addr

  ; ; If low order wraps around, updates high order
  ; CPX #$00
  ; BEQ update_snake_tail_high_order_byte
  ;;;;;

  ;;;;;;;
  JSR add_snake_to_its_direction
  

  JSR print_new_snake_head
  ;;;;;; 

  RTS

add_snake_to_its_direction:
  CLC
  LDX snake_direction_addr

  CPX #snake_up
  BEQ add_snake_up

  CPX #snake_right
  BEQ add_snake_right

  CPX #snake_bottom
  BEQ add_snake_bottom

  CPX #snake_left
  BEQ add_snake_left

  RTS

add_snake_up:
  LDA snake_head_low_byte_addr
  SEC
  SBC #$20  ; 32 in decimal
  STA snake_head_low_byte_addr

  BCS dec_snake_head_high_order_byte

  RTS

add_snake_right:
  ; updates new head
  LDX snake_head_low_byte_addr
  INX
  STX snake_head_low_byte_addr

  ; If low order wraps around, updates high order
  CPX #$00
  BEQ inc_snake_head_high_order_byte
  RTS

add_snake_bottom:
  LDA snake_head_low_byte_addr
  CLC
  ADC #$20  ; 32 in decimal
  STA snake_head_low_byte_addr

  BCS inc_snake_head_high_order_byte  ; Branch if Carry Set

  RTS

add_snake_left:
  ; updates new head
  LDX snake_head_low_byte_addr
  DEX
  STX snake_head_low_byte_addr

  ; If low order wraps around, updates high order
  CPX #$ff
  BEQ dec_snake_head_high_order_byte
  RTS

print_new_snake_head:
  LDA #snake_color                  ; loads the snake color (white) into the accumulator
  LDY #$00
  STA (snake_head_low_byte_addr), Y
  RTS

inc_snake_head_high_order_byte:
  LDX snake_head_high_byte_addr
  INX
  STX snake_head_high_byte_addr

  RTS

dec_snake_head_high_order_byte:
  LDX snake_head_high_byte_addr
  DEX
  STX snake_head_high_byte_addr

  RTS