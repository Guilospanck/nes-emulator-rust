LDA #$02 ; Loads accumulator with 0x02

TAX 	 ; Transfers 0x02 to Register X

LDA #$01 ; Loads accumulator with 0x01

STA $03	 ; Sets accumulator address to 0x03
STX $04  ; Sets Register X addr to 0x04

ADC $04  ; Adds accumulator to the value of addr 0x04, which is 0x02

TAX 	 ; Transfers this value (0x03) to Register X

SEC	     ; for correct next subtraction with SBC
SBC #$01 ; Subtracts the value 0x01, resulting in 0x02

TAY	     ; Transfers 0x02 to Register Y 
