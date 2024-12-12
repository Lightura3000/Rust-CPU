```
Fixed width:
FLEX - Fixed Length Execution
FINE - Fixed Instruction Narrow Execution
FRIC - Fixed Register Instruction Core
FALCON (Fixed ALigned COmputatioN)

Variable width:
VETRA (short, crisp, and could imply vector adaptability)
VARICORE (Variable Core—emphasizing the shifting instruction width at the heart)
VALCON (VAriable Length COmputatiON)
```

```
- **Opcode:** `0x`
- **Description:** TODO
- **Pseudocode:** `TODO`
- **Encoding:** `0x`
```

## No Operation (`nop`)
- **Opcode:** `0x00`
- **Description:** Does nothing
- **Encoding:** `0x00000000`
- **Usage:** Used for timing, synchronization, or padding

## Add (`add`)
- **Opcode:** `0x01`
- **Description:** Adds values from two registers
- **Pseudocode:** `rA = rB + rC`
- **Encoding:** `0x01ABC000`


- **Opcode:** `0x02`
- **Description:** Adds a register to a 16 bit immediate
- **Pseudocode:** `rA = rB + imm`
- **Encoding:** `0x02ABIIII`

## Subtract (`sub`)
- **Opcode:** `0x03`
- **Description:** Subtracts second register from first
- **Pseudocode:** `rA = rB - rC`
- **Encoding:** `0x03ABC000`


- **Opcode:** `0x04`
- **Description:** Subtracts 16 bit immediate from register
- **Pseudocode:** `rA = rB - imm`
- **Encoding:** `0x04ABIIII`


- **Opcode:** `0x05`
- **Description:** Subtracts register from 16 bit immediate
- **Pseudocode:** `rA = imm - rB`
- **Encoding:** `0x05ABIIII`


## Multiply (`mul`)
- **Opcode:** `0x06`
- **Description:** Multiplies two registers
- **Pseudocode:** `rA = rB * rC`
- **Encoding:** `0x06ABC000`


- **Opcode:** `0x07`
- **Description:** Multiplies a register by a 16 bit immediate
- **Pseudocode:** `rA = rB * imm`
- **Encoding:** `0x07ABIIII`

## Unsigned Division (`div`)
- **Opcode:** `0x08`
- **Description:** Divides first register by second (unsigned)
- **Pseudocode:** `rA = rB / rC`
- **Encoding:** `0x08ABC000`


- **Opcode:** `0x09`
- **Description:** Divides first register by a 16 bit immediate (unsigned)
- **Pseudocode:** `rA = rB / imm`
- **Encoding:** `0x`


- **Opcode:** `0x0A`
- **Description:** Divides a 16 bit immediate by a register (unsigned)
- **Pseudocode:** `rA = imm / rB`
- **Encoding:** `0x0aABIIII`

## Signed Division (`sdiv`)
- **Opcode:** `0x0B`
- **Description:** Divides first register by second (signed)
- **Pseudocode:** `rA = rB / rC`
- **Encoding:** `0x0bABC000`


- **Opcode:** `0x0C`
- **Description:** Divides first register by a 16 bit immediate (signed)
- **Pseudocode:** `rA = rB / imm`
- **Encoding:** `0x0cABIIII`


- **Opcode:** `0x0D`
- **Description:** Divides a 16 bit immediate by a register (signed)
- **Pseudocode:** `rA = imm / rB`
- **Encoding:** `0x0dABIIII`

## Bitwise AND (`and`)
- **Opcode:** `0x0E`
- **Description:** Performs bitwise AND between two registers
- **Pseudocode:** `rA = rB & rC`
- **Encoding:** `0x0eABC000`

## Bitwise OR (`or`)
- **Opcode:** `0x0F`
- **Description:** Performs bitwise OR between two registers
- **Pseudocode:** `rA = rB | rC`
- **Encoding:** `0x0fABC000`

## Bitwise XOR (`xor`)
- **Opcode:** `0x10`
- **Description:** Performs bitwise XOR between two registers
- **Pseudocode:** `rA = rB ^ rC`
- **Encoding:** `0x10ABC000`

## Bitwise NOT (`not`)
- **Opcode:** `0x11`
- **Description:** Performs bitwise NOT on a register
- **Pseudocode:** `rA = !rB`
- **Encoding:** `0x11A00000`

## Bitwise NAND (`nand`)
- **Opcode:** `0x12`
- **Description:** Performs bitwise NAND between two registers
- **Pseudocode:** `rA = !(rB & rC)`
- **Encoding:** `0x12ABC000`

## Bitwise NOR (`nor`)
- **Opcode:** `0x13`
- **Description:** Performs bitwise NOR
- **Pseudocode:** `rA = !(rB | rC)`
- **Encoding:** `0x13ABC000`

## Bitwise XNOR (`xnor`)
- **Opcode:** `0x14`
- **Description:** Performs bitwise XNOR
- **Pseudocode:** `rA = !(rB ^ rC)`
- **Encoding:** `0x14ABC000`

## Modulo (`mod`)
- **Opcode:** `0x15`
- **Description:** Calculate modulo between two registers (unsigned)
- **Pseudocode:** `rA = rB % rC`
- **Encoding:** `0x15ABC000`


- **Opcode:** `0x16`
- **Description:** Calculate modulo between a register and a 16 bit immediate (unsigned)
- **Pseudocode:** `rA = rB % imm`
- **Encoding:** `0x16ABIIII`


- **Opcode:** `0x17`
- **Description:** Calculate modulo between a 16 bit immediate and a register (unsigned)
- **Pseudocode:** `rA = imm % rB`
- **Encoding:** `0x17ABIIII`

## Signed modulo (`smod`)
- **Opcode:** `0x18`
- **Description:** Calculate modulo between two registers (signed)
- **Pseudocode:** `rA = rB % rC`
- **Encoding:** `0x18ABC000`


- **Opcode:** `0x19`
- **Description:** Calculate modulo between a register and a 16 bit immediate (signed)
- **Pseudocode:** `rA = rB % imm`
- **Encoding:** `0x19ABIIII`


- **Opcode:** `0x1A`
- **Description:** Calculate modulo between a 16 bit immediate and a register (signed)
- **Pseudocode:** `rA = imm % rB`
- **Encoding:** `0x1AABIIII`

## Right shift (`rsh`)
- **Opcode:** `0x1B`
- **Description:** Right shifts bits in a register by a specified amount
- **Pseudocode:** `rA = rA >> rC`
- **Encoding:** `0x1bABC000`


- **Opcode:** `0x1C`
- **Description:** Right shifts bits in a register by an 8 bit immediate
- **Pseudocode:** `rA = rA >> imm`
- **Encoding:** `0x1cAB00II`

## Left shift (`lsh`)
- **Opcode:** `0x1D`
- **Description:** Left shifts bits in a register by a specified amount
- **Pseudocode:** `rA = rA << rC`
- **Encoding:** `0x1dABC000`


- **Opcode:** `0x1e`
- **Description:** Left shifts bits in a register by an 8 bit immediate
- **Pseudocode:** `rA = rA << imm`
- **Encoding:** `0x1eAB00II`

## Right roll (`ror`)
- **Opcode:** `0x1F`
- **Description:** Rotates a register to the right once
- **Pseudocode:** `rA = rA.rotate_right(1)`
- **Encoding:** `0x1fAB0000`

## Left roll (`rol`)
- **Opcode:** `0x20`
- **Description:** Rotates a register to the left once
- **Pseudocode:** `rA = rA.rotate_left(1)`
- **Encoding:** `0x20AB0000`

## Move (`mov`)
- **Opcode:** `0x21`
- **Description:** Moves a value between registers
- **Pseudocode:** `rA = rB`
- **Encoding:** `0x21AB0000`

## Load immediate (`ldi`)
- **Opcode:** `0x22`
- **Description:** Loads a 16-bit immediate into a 16-bit chunk of a register
- **Pseudocode:** `TODO`
- **Encoding:** `0010 0010 AAAA 00SS IIII IIII IIII IIII`
- **Info:** `S` in the `encoding` signifies which chunk is addressed (0 = least significant, 4 = most significant)

## TODO UNUSED OPCODE | TODO UNUSED OPCODE | TODO UNUSED OPCODE | TODO UNUSED OPCODE | TODO UNUSED OPCODE | TODO UNUSED OPCODE
- **Opcode:** `0x23`
- **Description:** TODO
- **Pseudocode:** `TODO`
- **Encoding:** `0x23`

## Load register (`ldr`)
- **Opcode:** `0x24`
- **Description:** Loads a byte of a register from memory
- **Pseudocode:** `rA = memory[rB] << S`
- **Encoding:** `0010 0100 AAAA BBBB 0000 0000 0000 0SSS`
- **Info:** `S` in the `encoding` signifies which byte is addressed (0 = least significant, 7 = most significant)


- **Opcode:** `0x25`
- **Description:** Loads a register from memory addressed by 16 bits
- **Pseudocode:** `rA = memory[imm] << S`
- **Encoding:** `0010 0101 AAAA 0SSS IIII IIII IIII IIII`
- **Info:** `S` in the `encoding` signifies which byte is addressed (0 = least significant, 7 = most significant)

## Store register (`str`)
- **Opcode:** `0x26`
- **Description:** Stores a byte from a register into memory
- **Pseudocode:** `TODO`
- **Encoding:** `0010 0110 AAAA BBBB 0000 0000 0000 0SSS`
- **Info:** `S` in the `encoding` signifies which byte is addressed (0 = least significant, 7 = most significant)


- **Opcode:** `0x27`
- **Description:** Stores a byte from a register into memory addressed by 16 bits
- **Pseudocode:** `TODO`
- **Encoding:** `0010 0111 AAAA 0SSS IIII IIII IIII IIII`
- **Info:** `S` in the `encoding` signifies which byte is addressed (0 = least significant, 7 = most significant)

## Push (`push`)
- **Opcode:** `0x28`
- **Description:** Pushes a register to the stack and increases the stack pointer
- **Pseudocode:** `TODO`
- **Encoding:** `0x28A00000`

## Pop (`pop`)
- **Opcode:** `0x29`
- **Description:** Pops from the stack to a register and decreases the stack pointer
- **Pseudocode:** `TODO`
- **Encoding:** `0x29A00000`

## Compare (`cmp`)
- **Opcode:** `0x2A`
- **Description:** Compares two registers and sets appropriate flags (unsigned)
- **Pseudocode:** `rA.cmp(&rB)`
- **Encoding:** `0x2aAB0000`


- **Opcode:** `0x2B`
- **Description:** Compares a register with a 16-bit immediate and sets appropriate flags (unsigned)
- **Pseudocode:** `rA.cmp(&imm)`
- **Encoding:** `0x2bA0IIII`


- **Opcode:** `0x2C`
- **Description:** Compares a 16-bit immediate with a register and sets appropriate flags (unsigned)
- **Pseudocode:** `imm.cmp(&rA)`
- **Encoding:** `0x2cA0IIII`

## Signed comparison (`scmp`)
- **Opcode:** `0x2D`
- **Description:** Compares two registers and sets appropriate flags (signed)
- **Pseudocode:** `rA.cmp(&rB)`
- **Encoding:** `0x2dAB0000`


- **Opcode:** `0x2E`
- **Description:** Compares a register with a 16-bit immediate and sets appropriate flags (signed)
- **Pseudocode:** `rA.cmp(&imm)`
- **Encoding:** `0x2eA0IIII`


- **Opcode:** `0x2F`
- **Description:** Compares a 16-bit immediate with a register and sets appropriate flags (signed)
- **Pseudocode:** `imm.cmp(&rA)`
- **Encoding:** `0x2FA0IIII`

## Branch (`b`)
- **Opcode:** `0x30`
- **Description:** Offsets the instruction pointer by a register (signed)
- **Pseudocode:** `TODO`
- **Encoding:** `0x30A00000`


- **Opcode:** `0x31`
- **Description:** Offsets the instruction pointer by a 16-bit immediate (signed)
- **Pseudocode:** `TODO`
- **Encoding:** `0x3100IIII`

## Branch if greater than (`bg`)
- **Opcode:** `0x32`
- **Description:** Offsets the instruction pointer by a register (signed) if the greater flag is set
- **Pseudocode:** `TODO`
- **Encoding:** `0x32A00000`


- **Opcode:** `0x33`
- **Description:** Offsets the instruction pointer by a 16-bit immediate (signed) if the greater flag is set
- **Pseudocode:** `TODO`
- **Encoding:** `0x3300IIII`

## Branch if equal (`be`)
- **Opcode:** `0x34`
- **Description:** Offsets the instruction pointer by a register (signed) if the equal flag is set
- **Pseudocode:** `TODO`
- **Encoding:** `0x34A00000`


- **Opcode:** `0x35`
- **Description:** Offsets the instruction pointer by a 16-bit immediate (signed) if the equal flag is set
- **Pseudocode:** `TODO`
- **Encoding:** `0x3500IIII`

## Branch if smaller (`bs`)
- **Opcode:** `0x36`
- **Description:** Offsets the instruction pointer by a register (signed) if the smaller flag is set
- **Pseudocode:** `TODO`
- **Encoding:** `0x36A00000`


- **Opcode:** `0x37`
- **Description:** Offsets the instruction pointer by a 16-bit immediate (signed) if the smaller flag is set
- **Pseudocode:** `TODO`
- **Encoding:** `0x3700IIII`


### `bge` - Branch if greater than or equal
### `bne` - Branch if not equal
### `bse` - Branch if smaller or equal
### `int` - Trigger a software interrupt


# Pseudoinstructions
inc, dec
