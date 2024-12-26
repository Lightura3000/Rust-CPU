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
- **Description:** Right shifts bits in a register by a 6 bit immediate
- **Pseudocode:** `rA = rA >> imm`
- **Encoding:** `0001 1100 AAAA BBBB 0000 0000 00II IIII`

## Left shift (`lsh`)
- **Opcode:** `0x1D`
- **Description:** Left shifts bits in a register by a specified amount
- **Pseudocode:** `rA = rA << rC`
- **Encoding:** `0x1dABC000`


- **Opcode:** `0x1E`
- **Description:** Left shifts bits in a register by a 6 bit immediate
- **Pseudocode:** `rA = rA << imm`
- **Encoding:** `0001 1110 AAAA BBBB 0000 0000 00II IIII`

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
- **Encoding:** `0010 0010 AAAA 00SS IIII IIII IIII IIII`
- **Info:** `S` in the `encoding` signifies which chunk is addressed (0 = least significant, 3 = most significant)

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
- **Encoding:** `0010 0110 AAAA BBBB 0000 0000 0000 0SSS`
- **Info:** `S` in the `encoding` signifies which byte is addressed (0 = least significant, 7 = most significant)


- **Opcode:** `0x27`
- **Description:** Stores a byte from a register into memory addressed by 16 bits
- **Encoding:** `0010 0111 AAAA 0SSS IIII IIII IIII IIII`
- **Info:** `S` in the `encoding` signifies which byte is addressed (0 = least significant, 7 = most significant)

## Push (`push`)
- **Opcode:** `0x28`
- **Description:** Pushes a register to the stack and increases the stack pointer
- **Encoding:** `0x28A00000`

## Pop (`pop`)
- **Opcode:** `0x29`
- **Description:** Pops from the stack to a register and decreases the stack pointer
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
- **Encoding:** `0x30A00000`


- **Opcode:** `0x31`
- **Description:** Offsets the instruction pointer by a 16-bit immediate (signed)
- **Encoding:** `0x3100IIII`

## Branch if greater than (`bg`)
- **Opcode:** `0x32`
- **Description:** Offsets the instruction pointer by a register (signed) if the greater flag is set
- **Encoding:** `0x32A00000`


- **Opcode:** `0x33`
- **Description:** Offsets the instruction pointer by a 16-bit immediate (signed) if the greater flag is set
- **Encoding:** `0x3300IIII`

## Branch if equal (`be`)
- **Opcode:** `0x34`
- **Description:** Offsets the instruction pointer by a register (signed) if the equal flag is set
- **Encoding:** `0x34A00000`


- **Opcode:** `0x35`
- **Description:** Offsets the instruction pointer by a 16-bit immediate (signed) if the equal flag is set
- **Encoding:** `0x3500IIII`

## Branch if smaller (`bs`)
- **Opcode:** `0x36`
- **Description:** Offsets the instruction pointer by a register (signed) if the smaller flag is set
- **Encoding:** `0x36A00000`


- **Opcode:** `0x37`
- **Description:** Offsets the instruction pointer by a 16-bit immediate (signed) if the smaller flag is set
- **Encoding:** `0x3700IIII`

## Branch if greater than or equal (`bge`) 
- **Opcode:** `0x38`
- **Description:** Offsets the instruction pointer by a 16-bit immediate (signed) if the greater flag is not set
- **Encoding:** `0x3800IIII`

## Branch if not equal (`bne`)
- **Opcode:** `0x39`
- **Description:** Offsets the instruction pointer by a 16-bit immediate (signed) if the equal flag is not set
- **Encoding:** `0x3900IIII`

## Branch if smaller or equal (`bse`)
- **Opcode:** `0x3A`
- **Description:** Offsets the instruction pointer by a 16-bit immediate (signed) if the smaller flag is not set
- **Encoding:** `0x3A00IIII`

## Integer to floating point (`ftoi`)
- **Opcode:** `0x3B`
- **Description:** Converts an integer in B to a float in A
- **Encoding:** `0x3bAB0000`
- **Info:** Since a float is 32 bits long, only the least significant 32 bits of A are affected

## Floating point to integer (`itof`)
- **Opcode:** `0x3C`
- **Description:** Converts a float in B to an integer in A
- **Encoding:** `0x3cAB0000`

## Floating point add (`fadd`)
- **Opcode:** `0x3D`
- **Description:** Adds two floats from registers B and C and puts result in A
- **Encoding:** `0x3dABC000`
- **Info:** Since a float is 32 bits long, only the least significant 32 bits of A are affected

## Floating point subtract (`fsub`)
- **Opcode:** `0x3E`
- **Description:** Subtracts the float in B from the float in C and puts the result in A
- **Encoding:** `0x3eABC000`
- **Info:** Since a float is 32 bits long, only the least significant 32 bits of A are affected

## Floating point multiply (`fmul`)
- **Opcode:** `0x3F`
- **Description:** Multiplies the floats from registers B and C and puts the result in A
- **Encoding:** `0x3fABC000`
- **Info:** Since a float is 32 bits long, only the least significant 32 bits of A are affected

## Floating point divide (`fdiv`)
- **Opcode:** `0x40`
- **Description:** Divides the float in A by the float in C and stores the result in C
- **Encoding:** `0x40ABC000`
- **Info:** Since a float is 32 bits long, only the least significant 32 bits of A are affected

## Floating point negate (`fneg`)
- **Opcode:** `0x41`
- **Description:** Negates a the float in A
- **Encoding:** `0x41A00000`
- **Info:** Since a float is 32 bits long, only the least significant 32 bits of A are affected

## Floating point compare (`fcmp`)
- **Opcode:** `0x42`
- **Description:** Compares the float values in A and C and sets appropriate flags
- **Encoding:** `0x42AB0000`
- **Info:** Since a float is 32 bits long, only the least significant 32 bits of A are affected

## Floating point power (`fpow`)
- **Opcode:** `0x43`
- **Description:** Raises A to the power B and stores the result in A
- **Encoding:** `0x43AB0000`
- **Info:** Since a float is 32 bits long, only the least significant 32 bits of A are affected

## Floating point root (`froot`)
- **Opcode:** `0x44`
- **Description:** Takes the Bth root of the float in A and stores the result in A
- **Encoding:** `0x44AB0000`
- **Info:** Since a float is 32 bits long, only the least significant 32 bits of A are affected

## Floating point logarithm (`flog`)
- **Opcode:** `0x45`
- **Description:** Calculates the logarithm of A with base B and stores the result in A
- **Encoding:** `0x45AB0000`
- **Info:** Since a float is 32 bits long, only the least significant 32 bits of A are affected

## Floating point absolute value (`fabs`)
- **Opcode:** `0x46`
- **Description:** Calculates the absolute value of A and stores the result in A
- **Encoding:** `0x46A00000`
- **Info:** Since a float is 32 bits long, only the least significant 32 bits of A are affected

## Floating point sin (`fsin`)
- **Opcode:** `0x47`
- **Description:** Calculates the sine of B and stores the result in A
- **Encoding:** `0x47AB0000`
- **Info:** Since a float is 32 bits long, only the least significant 32 bits of A are affected

## Floating point cos (`fcos`)
- **Opcode:** `0x48`
- **Description:** Calculates the cosine of B and stores the result in A
- **Encoding:** `0x48AB0000`
- **Info:** Since a float is 32 bits long, only the least significant 32 bits of A are affected

## Floating point tan (`ftan`)
- **Opcode:** `0x49`
- **Description:** Calculates the tangent of B and stores the result in A
- **Encoding:** `0x49AB0000`
- **Info:** Since a float is 32 bits long, only the least significant 32 bits of A are affected

## Floating point inverse sin (`fasin`)
- **Opcode:** `0x4A`
- **Description:** Calculates the inverse sine of B and stores the result in A
- **Encoding:** `0x4aAB0000`
- **Info:** Since a float is 32 bits long, only the least significant 32 bits of A are affected

## Floating point inverse cos (`facos`)
- **Opcode:** `0x4B`
- **Description:** Calculates the inverse cosine of B and stores the result in A
- **Encoding:** `0x4bAB0000`
- **Info:** Since a float is 32 bits long, only the least significant 32 bits of A are affected

## Floating point inverse tan (`fatan`)
- **Opcode:** `0x4C`
- **Description:** Calcuates the inverse tangent of B and stores the result in A
- **Encoding:** `0x4cAB0000`
- **Info:** Since a float is 32 bits long, only the least significant 32 bits of A are affected

## Floating point round down (`ffloor`)
- **Opcode:** `0x4D`
- **Description:** Calculates the floor of A and stores the result in A (still as a float)
- **Encoding:** `0x4dA0000`
- **Info:** Since a float is 32 bits long, only the least significant 32 bits of A are affected

## Floating point round up (`fceil`)
- **Opcode:** `0x4E`
- **Description:** Calculates the ceiling of A and stores the result in A (still as a float)
- **Encoding:** `0x4eA00000`
- **Info:** Since a float is 32 bits long, only the least significant 32 bits of A are affected

## Floating point round (`fround`)
- **Opcode:** `0x4F`
- **Description:** Rounds A and stores the result in A (still as a float)
- **Encoding:** `0x4fA00000`
- **Info:** Since a float is 32 bits long, only the least significant 32 bits of A are affected

## Floating point load infinity (`finf`)
- **Opcode:** `0x50`
- **Description:** Loads the floating point value for infinity into A
- **Encoding:** `0x50A00000`
- **Info:** Since a float is 32 bits long, only the least significant 32 bits of A are affected

## Floating point load NaN (`fnan`)
- **Opcode:** `0x51`
- **Description:** Loads the floating point value for NaN into A
- **Encoding:** `0x51A00000`
- **Info:** Since a float is 32 bits long, only the least significant 32 bits of A are affected

## Integer to double precision (`itod`)
- **Opcode:** `0x52`
- **Description:** Converts the integer value in A to a double 
- **Encoding:** `0x52A00000`

## Double precision to integer (`dtoi`)
- **Opcode:** `0x53`
- **Description:** Converts the double in A to an integer
- **Encoding:** `0x53A00000`

## Float point to double precision (`ftod`)
- **Opcode:** `0x54`
- **Description:** Converts the float in A to a double
- **Encoding:** `0x54A00000`

## Double precision to floating point (`dtof`)
- **Opcode:** `0x55`
- **Description:** Converts the double in A to a float
- **Encoding:** `0x55A00000`

## Double precision add (`dadd`)
- **Opcode:** `0x56`
- **Description:** Adds two doubles from registers B and C and puts result in A
- **Encoding:** `0x56ABC000`

## Double precision subtract (`dsub`)
- **Opcode:** `0x57`
- **Description:** Subtracts the double in B from the double in C and puts the result in A
- **Encoding:** `0x57ABC000`

## Double precision multiply (`dmul`)
- **Opcode:** `0x58`
- **Description:** Multiplies the doubles from registers B and C and puts the result in A
- **Encoding:** `0x58ABC000`

## Double precision divide (`ddiv`)
- **Opcode:** `0x59`
- **Description:** Divides the double in A by the double in C and stores the result in C
- **Encoding:** `0x59ABC000`

## Double precision negate (`dneg`)
- **Opcode:** `0x5A`
- **Description:** Negates a the double in A
- **Encoding:** `0x5aA00000`

## Double precision compare (`dcmp`)
- **Opcode:** `0x5B`
- **Description:** Compares the double values in A and C and sets appropriate flags
- **Encoding:** `0x5bAB0000`

## Double precision power (`dpow`)
- **Opcode:** `0x5C`
- **Description:** Raises A to the power B and stores the result in A
- **Encoding:** `0x5cAB0000`

## Double precision root (`droot`)
- **Opcode:** `0x5D`
- **Description:** Takes the Bth root of the double in A and stores the result in A
- **Encoding:** `0x5dAB0000`

## Double precision logarithm (`dlog`)
- **Opcode:** `0x5E`
- **Description:** Calculates the logarithm of A with base B and stores the result in A
- **Encoding:** `0x5eAB0000`

## Double precision absolute value (`dabs`)
- **Opcode:** `0x5F`
- **Description:** Calculates the absolute value of A and stores the result in A
- **Encoding:** `0x5fA00000`

## Double precision sin (`dsin`)
- **Opcode:** `0x60`
- **Description:** Calculates the sine of B and stores the result in A
- **Encoding:** `0x60AB0000`

## Double precision cos (`dcos`)
- **Opcode:** `0x61`
- **Description:** Calculates the cosine of B and stores the result in A
- **Encoding:** `0x61AB0000`

## Double precision tan (`dtan`)
- **Opcode:** `0x62`
- **Description:** Calculates the tangent of B and stores the result in A
- **Encoding:** `0x62AB0000`

## Double precision inverse sin (`dasin`)
- **Opcode:** `0x63`
- **Description:** Calculates the inverse sine of B and stores the result in A
- **Encoding:** `0x63AB0000`

## Double precision inverse cos (`dacos`)
- **Opcode:** `0x64`
- **Description:** Calculates the inverse cosine of B and stores the result in A
- **Encoding:** `0x64AB0000`

## Double precision inverse tan (`datan`)
- **Opcode:** `0x65`
- **Description:** Calcuates the inverse tangent of B and stores the result in A
- **Encoding:** `0x65AB0000`

## Double precision round down (`dfloor`)
- **Opcode:** `0x66`
- **Description:** Calculates the floor of A and stores the result in A (still as a double)
- **Encoding:** `0x66A0000`

## Double precision round up (`dceil`)
- **Opcode:** `0x67`
- **Description:** Calculates the ceiling of A and stores the result in A (still as a double)
- **Encoding:** `0x67A00000`

## Double precision round (`dround`)
- **Opcode:** `0x68`
- **Description:** Rounds A and stores the result in A (still as a double)
- **Encoding:** `0x68A00000`

## Double precision load infinity (`dinf`)
- **Opcode:** `0x69`
- **Description:** Loads the double precision value for infinity into A
- **Encoding:** `0x69A00000`

## Double precision load NaN (`dnan`)
- **Opcode:** `0x6A`
- **Description:** Loads the double precision value for NaN into A
- **Encoding:** `0x6aA00000`


## Trigger a software interrupt (`int`) (TODO)


# Pseudoinstructions (TODO)
inc, dec
