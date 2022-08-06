  LDA #$02 ; Load Accumulator (LDA): Loads accumulator with 0x02

  TAX 	   ; Transfers Accumulator to X (TAX): Transfers 0x02 to Register X

  LDA #$01 ; Load Accumulator (LDA): Loads accumulator with 0x01

  STA $03	 ; Store Accumulator (STA): stores the value of the Accumulator into memory at the address 0x03
  STX $04  ; Store X Register (STX): stores the value of the X Register into memory at the address 0x04

  ADC $04  ; Add With Carry (ADC): Adds accumulator to the value of addr 0x04, which is 0x02

  TAX 	   ; Transfers Accumulator to X (TAX): Transfers this value (0x03) to Register X

  SEC	     ; Set Carry Flag (SEC): sets the carry flag to one in order to do the right subtraction with SBC
  SBC #$01 ; Subtract With Carry (SBC): Subtracts the value 0x01, resulting in 0x02

  TAY	     ; Transfers Accumulator to Y (TAY): Transfers 0x02 to Register Y 
