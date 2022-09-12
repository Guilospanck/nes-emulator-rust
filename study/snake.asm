; This is a simple snake game built in assembly language 6502.

;; System variables
define system_last_key $ff ; gets the last pressed key
define sys_random $fe ; random system value

;; ASCII WASD Keys
define w_key $77 ; w key in ascii hex format
define a_key $61 ; a key in ascii hex format
define s_key $73 ; s key in ascii hex format
define d_key $64 ; d key in ascii hex format

;; Snake Properties
define snake_length_addr $00                     ; address 0x02 will have the data for the snake length
define snake_direction_addr $01                  ; address 0x03 will have the data for the snake direction

define snake_head_low_byte_addr $10              ; address 0x04 will have the data for the snake head low byte
define snake_head_high_byte_addr $11             ; address 0x05 will have the data for the snake head high byte
; define snake_tail_low_byte_addr $04              ; address 0x06 will have the data for the snake tail low byte
; define snake_tail_high_byte_addr $05             ; address 0x07 will have the data for the snake tail high byte

define snake_color 4                             ; defines the color of the snake (purple)
define snake_bg_color 0                          ; defines the bg color (black)
define snake_length 4                            ; initial snake length value

;; Apple properties
define apple_color 1
define apple_location_low_byte_addr $02
define apple_location_high_byte_addr $03

;; Possible snake directions
define snake_up 1
define snake_right 2
define snake_bottom 3
define snake_left 4

; Main
  JSR init_snake
  JSR init_apple
  JSR loop

init_snake:
  LDA #snake_length ; snake_length
  STA snake_length_addr

  LDA #snake_right ; snake direction
  STA snake_direction_addr

  ; draws snake
  JSR save_init_snake_body

  LDA snake_length_addr
  ASL ; arithmetic shift left (multiply by 2)
  TAX
  LDA #snake_color
  JSR draw_snake

  RTS

save_init_snake_body:
  LDA #$08
  STA $10 ; snake_head_low_byte_addr
  LDA #$04
  STA $11 ; snake_head_high_byte_addr

  LDA #$07
  STA $12
  LDA #$04
  STA $13

  LDA #$06
  STA $14
  LDA #$04
  STA $15

  LDA #$05
  STA $16 ; snake_tail_low_byte_addr
  LDA #$04
  STA $17 ; snake_tail_high_byte_addr

  RTS

draw_snake:
  STA (snake_head_low_byte_addr, X)
  DEX
  DEX
  BPL draw_snake

  RTS

init_apple:
  ; updates apple location
  LDX #$09
  STX apple_location_low_byte_addr
  LDX #$03
  STX apple_location_high_byte_addr

  ; draws apple
  JSR draw_apple

  RTS

draw_apple:
  LDY #$00
  LDA #apple_color
  STA (apple_location_low_byte_addr), Y
  RTS

loop:
  JSR read_keys

  ; moves snake  
  JSR move_snake

  ; delay loop
  LDX #$00
  JSR delay_loop

  ; infinite loop
  LDA #$01
  CMP #$02
  BNE loop

delay_loop:
  INX
  CPX #$ff
  BNE delay_loop
  RTS

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
  JSR load_x_register_with_snake_length_mult_by_2_minus_2
  JSR clear_snake_tail

  JSR shift_snake_position

  JSR move_snake_to_its_direction  

  JSR print_new_snake_head

  RTS  

load_x_register_with_snake_length_mult_by_2_minus_2:
  LDA snake_length_addr
  ASL
  TAX
  DEX
  DEX

  RTS

clear_snake_tail:
  LDA #snake_bg_color
  STA (snake_head_low_byte_addr, X)

  RTS

shift_snake_position:
  DEX
  LDA snake_head_low_byte_addr, X
  INX
  INX
  STA snake_head_low_byte_addr, X
  DEX
  DEX
  CPX #$00
  BNE shift_snake_position 

  RTS

move_snake_to_its_direction:
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
  JSR subtracts_snake_head_low_byte

  BCC dec_snake_head_high_order_byte        ; Branch if Carry Clear

  ;; If high order greater than 0x50 or lesser than 0x20, wraps
  JSR return_to_the_perimeter_of_the_board_top_bottom
  RTS

dec_snake_head_high_order_byte:
  LDX snake_head_high_byte_addr
  DEX
  STX snake_head_high_byte_addr

  RTS

subtracts_snake_head_low_byte:
  LDA snake_head_low_byte_addr
  SEC                                       ; Set Carry Flag
  SBC #$20  ; 32 in decimal
  STA snake_head_low_byte_addr
  RTS

add_snake_right:
  ; updates new head
  LDX snake_head_low_byte_addr
  INX
  STX snake_head_low_byte_addr

  ;; If low order wraps around, get back to the beginning of the line
  JSR return_to_the_beginning_of_the_line_right_direction

  JSR return_to_the_perimeter_of_the_board_left_right

  RTS

add_snake_bottom:
  JSR adds_snake_head_low_byte

  BCS inc_snake_head_high_order_byte  ; Branch if Carry Set

  ;; If high order greater than 0x50 or lesser than 0x20, wraps
  JSR return_to_the_perimeter_of_the_board_top_bottom 

  RTS

adds_snake_head_low_byte:
  LDA snake_head_low_byte_addr
  CLC                                       ; Clear Carry Flag
  ADC #$20  ; 32 in decimal
  STA snake_head_low_byte_addr
  RTS

add_snake_left:
  ; updates new head
  LDX snake_head_low_byte_addr
  DEX
  STX snake_head_low_byte_addr

  ;; If low order wraps around, return to the beginning of the line
  JSR return_to_the_beginning_of_the_line_left_direction

  JSR return_to_the_perimeter_of_the_board_left_right
  RTS

print_new_snake_head:
  LDY #$00

  ; verifies if snake it's bitting its own body
  LDA (snake_head_low_byte_addr), Y
  CMP #snake_color
  BEQ end_game

  ; verifies if an apple was found
  CMP #apple_color
  BEQ update_snake_length_and_randomize_apple

  LDA #snake_color                  ; loads the snake color into the accumulator
  STA (snake_head_low_byte_addr), Y
  RTS

update_snake_length_and_randomize_apple:
  LDX snake_length_addr
  INX
  STX snake_length_addr

  JSR load_x_register_with_snake_length_mult_by_2_minus_2  
  JSR shift_snake_position

  JSR randomize_apple

  RTS

randomize_apple:
  JSR clear_apple
  JSR verify_random_apple_position

  RTS

clear_apple:  
  LDY #$00
  LDA #snake_color
  STA (apple_location_low_byte_addr), Y

  RTS

verify_random_apple_position:
  LDA sys_random
  STA apple_location_low_byte_addr

  LDA sys_random
  AND #$03 ; mask the 2 lowest bits (0000 0011 = 0x03)
  CLC
  ADC #2
  STA apple_location_high_byte_addr

  ; verifies if apple is at the same spot as snake
  LDY #$00
  LDA (apple_location_low_byte_addr), Y
  CMP #snake_color
  BEQ verify_random_apple_position

  JSR draw_apple

  RTS

end_game:
  BRK

inc_snake_head_high_order_byte:
  LDX snake_head_high_byte_addr
  INX
  STX snake_head_high_byte_addr

  RTS

return_to_the_perimeter_of_the_board_top_bottom:
  LDX snake_head_high_byte_addr

  CPX #$06
  BEQ go_back_to_high_order_02_top_bottom

  CPX #$01
  BEQ go_back_to_high_order_05_top_bottom

  RTS

go_back_to_high_order_02_top_bottom:
  JSR subtracts_snake_head_low_byte
  LDX #$02
  STX snake_head_high_byte_addr
  RTS

go_back_to_high_order_05_top_bottom:
  JSR adds_snake_head_low_byte
  LDX #$05
  STX snake_head_high_byte_addr
  RTS

return_to_the_perimeter_of_the_board_left_right:
  LDX snake_head_high_byte_addr

  CPX #$06
  BEQ go_back_to_high_order_02_left_right

  CPX #$01
  BEQ go_back_to_high_order_05_left_right

  RTS

go_back_to_high_order_02_left_right:
  LDX #$02
  STX snake_head_high_byte_addr
  RTS

go_back_to_high_order_05_left_right:
  LDX #$05
  STX snake_head_high_byte_addr
  RTS

return_to_the_beginning_of_the_line_right_direction:
  ; right direction
  CPX #$00
  BEQ go_back_to_e0

  CPX #$20
  BEQ go_back_to_00

  CPX #$40
  BEQ go_back_to_20
  
  CPX #$60
  BEQ go_back_to_40
  
  CPX #$80
  BEQ go_back_to_60
  
  CPX #$A0
  BEQ go_back_to_80
  
  CPX #$C0
  BEQ go_back_to_a0
  
  CPX #$E0
  BEQ go_back_to_c0

  RTS  

return_to_the_beginning_of_the_line_left_direction:  
  ; left direction
  CPX #$ff
  BEQ go_back_to_1f

  CPX #$1f
  BEQ go_back_to_3f

  CPX #$3f
  BEQ go_back_to_5f

  CPX #$5f
  BEQ go_back_to_7f

  CPX #$7f
  BEQ go_back_to_9f

  CPX #$9f
  BEQ go_back_to_bf

  CPX #$bf
  BEQ go_back_to_df

  CPX #$df
  BEQ go_back_to_ff

  RTS  

; right direction
go_back_to_00:
  LDX #$00
  STX snake_head_low_byte_addr
  RTS

go_back_to_20:
  LDX #$20
  STX snake_head_low_byte_addr
  RTS

go_back_to_40:
  LDX #$40
  STX snake_head_low_byte_addr
  RTS

go_back_to_60:
  LDX #$60
  STX snake_head_low_byte_addr
  RTS

go_back_to_80:
  LDX #$80
  STX snake_head_low_byte_addr
  RTS

go_back_to_a0:
  LDX #$a0
  STX snake_head_low_byte_addr
  RTS

go_back_to_c0:
  LDX #$c0
  STX snake_head_low_byte_addr
  RTS

go_back_to_e0:
  LDX #$e0
  STX snake_head_low_byte_addr
  RTS

; left direction
go_back_to_1f:
  LDX #$1f
  STX snake_head_low_byte_addr
  RTS

go_back_to_3f:
  LDX #$3f
  STX snake_head_low_byte_addr
  RTS

go_back_to_5f:
  LDX #$5f
  STX snake_head_low_byte_addr
  RTS

go_back_to_7f:
  LDX #$7f
  STX snake_head_low_byte_addr
  RTS

go_back_to_9f:
  LDX #$9f
  STX snake_head_low_byte_addr
  RTS

go_back_to_bf:
  LDX #$bf
  STX snake_head_low_byte_addr
  RTS

go_back_to_df:
  LDX #$df
  STX snake_head_low_byte_addr
  RTS

go_back_to_ff:
  LDX #$ff
  STX snake_head_low_byte_addr
  RTS