<!-- TOC -->
  * [1. No operation](#1-no-operation)
  * [2. Arithmetic operations](#2-arithmetic-operations)
  * [3. Bitwise operations](#3-bitwise-operations)
  * [4. Shift & Rotate](#4-shift--rotate)
  * [5. Data movement, Memory, Stack](#5-data-movement-memory-stack)
  * [6. Comparison](#6-comparison)
  * [7. Branching](#7-branching)
  * [8. Conversions between integers, floats and doubles](#8-conversions-between-integers-floats-and-doubles)
  * [9. Floating point arithmetic](#9-floating-point-arithmetic)
  * [10. Double precision arithmetic](#10-double-precision-arithmetic)
<!-- TOC -->


## 1. No operation
**Assembly opcode:** `nop`

**Description:** Does nothing

```
0000 0000 0000 0000 0000 0000 0000 0000
^^^^
Opcode
```

## 2. Arithmetic operations
```
(a)
          Src reg1
          vvvv
0001 AAAA BBBB CCCC 0000 0000 0000 OOOO
^^^^  ^^^^     ^^^^                ^^^^
Opc   Dest reg  Src reg           Operation

(b)
          Src reg
          vvvv
0001 AAAA BBBB IIII IIII IIII IIII OOOO
^^^^ ^^^^      ^^^^-^^^^-^^^^-^^^^ ^^^^
Opc  Dest reg  Immediate           Operation
```

This instruction performs an arithmetic operation. Which one exactly depends on the operation (`O`) bits:
- `0000 (0)` Register addition (bit pattern `a`). Registers `B` and `C` get added, result is stored in `A`.
- `0001 (1)` Immediate addition (bit pattern `b`). Register `B` gets added to the `immediate`, result is stored in `A`.
- `0010 (2)` Register subtraction (bit pattern `a`). Register `C` gets subtracted from `B`, result is stored in `A`.
- `0011 (3)` Immediate subtraction (bit pattern `b`). The `immediate` gets subtracted from `B`, result is stored in `A`.
- `0100 (4)` Reverse immediate subtraction (bit pattern `b`). Register `B` gets subtracted from the `immediate`, result is stored in `A`.
- `0101 (5)` Register multiplication (bit pattern `a`). Registers `B` and `C` get multiplied, result is stored in `A`
- `0110 (6)` Immediate multiplication (bit pattern `b`). Register `B` gets multiplied with the `immediate`, result is stored in `A`.
- `0111 (7)` Unsigned register division (bit pattern `a`). Register `B` gets divided by `C`, result is stored in `A`.
- `1000 (8)` Unsigned immediate division (bit pattern `b`). Register `B` gets divided by the `immediate`, result is stored in `A`.
- `1001 (9)` Unsigned reverse immediate division (bit pattern `b`). The `immediate` gets divided by `B`, result is stored in `A`.
- `1010 (A)` Signed register division (bit pattern `a`). Register `B` gets divided by `C`, result is stored in `A`.
- `1011 (B)` Signed immediate division (bit pattern `b`). Register `B` gets divided by the `immediate`, result is stored in `A`.
- `1100 (C)` Signed reverse immediate division (bit pattern `b`). The `immediate` gets divided by `B`, result is stored in `A`.
- `1101 (D)` Unassigned. Using this will do nothing.
- `1110 (E)` Unassigned. Using this will do nothing.
- `1111 (F)` Unassigned. Using this will do nothing.


## 3. Bitwise operations
```
(a)
          Src reg1
          vvvv
0010 AAAA BBBB CCCC 0000 0000 0000 0OOO
^^^^ ^^^^      ^^^^                 ^^^
Opc  Dest reg  Src reg2             Operation

(b)
          Src reg
          vvvv
0010 AAAA BBBB 0000 0000 0000 0000 0OOO
^^^^ ^^^^                           ^^^
Opc  Dest reg                       Operation
```

This instruction performs a bitwise operation between two registers. Which one exactly depends on the operation (`O`) bits:
- `000 (0)` Bitwise AND (bit pattern `a`). Performs bitwise AND between `B` and `C`, result is stored in `A`.
- `001 (1)` Bitwise OR (bit pattern `a`). Performs bitwise OR between `B` and `C`, result is stored in `A`.
- `010 (2)` Bitwise XOR (bit pattern `a`). Performs bitwise XOR between `B` and `C`, result is stored in `A`.
- `011 (3)` Bitwise NAND (bit pattern `a`). Performs bitwise NAND between `B` and `C`, result is stored in `A`.
- `100 (4)` Bitwise NOR (bit pattern `a`). Performs bitwise NOR between `B` and `C`, result is stored in `A`.
- `101 (5)` Bitwise XNOR (bit pattern `a`). Performs bitwise XNOR between `B` and `C`, result is stored in `A`.
- `110 (6)` Bitwise NOT (bit pattern `b`). Performs bitwise NOT on `B`, result is stored in `A`.
- `111 (7)` Unassigned. Using this will do nothing.

## 4. Shift & Rotate
```
(a)
          Src reg1
          vvvv
0011 AAAA BBBB CCCC 0000 0000 0000 0OOO
^^^^ ^^^^      ^^^^                 ^^^
Opc  Dest reg  Src reg              Operation

(b)
          Src reg          Immediate
          vvvv             vv-vvvv
0011 AAAA BBBB 0000 0000 00II IIII 0OOO
^^^^ ^^^^                           ^^^
Opc  Dest reg                       Operation
```

This instruction performs a shift or rotation. Which one exactly depends on the operation (`O`) bits:
- `000 (0)` Right shift (bit pattern `a`). Register `B` is right-shifted by the value in `C`, result is stored in `A`.
- `001 (1)` Immediate right shift (bit pattern `b`). Register `B` is right-shifted by the `immediate`, result is stored in `A`. 
- `010 (2)` Left shift (bit pattern `a`). Register `B` is left-shifted by the value in `C`, result is stored in `A`.
- `011 (3)` Immediate left shift (bit pattern `b`). Register `B` is left-shifted by the `immediate`, result is stored in `A`.
- `100 (4)` Right roll (bit pattern `a`). Register `B` is right-rolled by the value in `C`, result is stored in `A`.
- `101 (5)` Immediate right roll (bit pattern `b`). Register `B` is right-rolled by the `immediate`, result is stored in `A`.
- `110 (6)` Left roll (bit pattern `a`). Register `B` is left-rolled by the value in `C`, result is stored in `A`.
- `111 (7)` Immediate left roll (bit pattern `b`). Register `B` is left-rolled by the `immediate`, result is stored in `A`.

## 5. Data movement, Memory, Stack
```
(a)
          Src reg
          vvvv
0100 AAAA BBBB 0000 0000 0000 0000 0OOO
^^^^ ^^^^                           ^^^
Opc  Dest reg                       Operation

(b)
          Immediate             Chunk
          vvvv-vvvv-vvvv-vvvv   vv
0100 AAAA IIII IIII IIII IIII 00CC 0OOO
^^^^ ^^^^                           ^^^
Opc  Dest reg                       Operation

(c)
          Mem src              Section
          vvvv                 vvv
0100 AAAA BBBB 0000 0000 0000 0SSS 0OOO
^^^^ ^^^^                           ^^^
Opc  Dest reg                       Operation

(d)
          Mem src              Section
          vvvv                 vvv
0100 AAAA BBBB 0000 0000 0000 0SSS 0OOO
^^^^ ^^^^                           ^^^
Opc  Dest reg                       Operation
```

This instruction is for moving data between registers, memory and stack operations. Which operation is executed exactly depends on the operation (`O`) bits:
- `000 (0)` Move registers (bit pattern `a`). Register `B` is moved into `A`.
- `001 (1)` Load immediate (bit pattern `b`). Loads an `immediate` into a 16-bit chunk of a register. The `C` bits signify which chunk is addressed (0 = least significant, 3 = most significant)
- `010 (2)` Load register (bit pattern `c`). Loads a byte from memory into a byte of a register. The `B` register will be the byte in memory and the byte will be loaded into a byte of register `A`. The `S` bits signify which section is addressed (0 = least significant, 7 = most significant)  
- `011 (3)` Load immediate address (bit pattern `d`). Loads a byte from memory addressed by the `immediate` into register `A`. The `S` bits signify which byte is addressed (0 = least significant, 7 = most significant)
- `100 (4)` Store register (bit pattern `c`). Stores a byte from register `A` into memory addressed by register `B`. The `S` bits signify which byte is addressed (0 = least significant, 7 = most significant)
- `101 (5)` Store immediate address (bit pattern `d`). Stores a byte from register `A` into memory addressed by the `immediate`. The `S` bits signify which byte is addressed (0 = least significant, 7 = most significant)
- `110 (6)` Push (bit pattern `a`). Pushes register `A` to the stack and increases the stack pointer
- `111 (7)` Pop (bit pattern `a`). Pops from the stack to register `A` and decreases the stack pointer

## 6. Comparison
```
(a)
0101 AAAA BBBB 0000 0000 0000 0000 0OOO
^^^^ ^^^^ ^^^^                      ^^^
Opc  Reg1 Reg2                      Comparison

(b)
          Immediate
          vvvv-vvvv-vvvv-vvvv
0101 AAAA IIII IIII IIII IIII 0000 0CCC
^^^^ ^^^^                           ^^^
Opc  Reg                            Comparison
```

This instruction performs comparisons between registers and immediates, setting appropriate flags based on the result. The comparison sets three flags:
- **Greater flag:** Set if first operand is greater than second operand
- **Equal flag:** Set if operands are equal
- **Smaller flag:** Set if first operand is smaller than second operand

Which comparison is performed depends on the comparison (`C`) bits:
- `000 (0)` Unsigned register comparison (bit pattern `a`). Compares register `A` with register `B` unsigned
- `001 (1)` Unsigned immediate comparison (bit pattern `b`). Compares register `A` with the `immediate` unsigned
- `010 (2)` Unsigned reverse immediate comparison (bit pattern `b`). Compares the `immediate` with register `A` unsigned
- `011 (3)` Signed register comparison (bit pattern `a`). Compares register `A` with register `B` signed
- `100 (4)` Signed immediate comparison (bit pattern `b`). Compares register `A` with the `immediate` signed
- `101 (5)` Signed reverse immediate comparison (bit pattern `b`). Compares the `immediate` with register `A` signed
- `110 (6)` Floating point comparison (bit pattern `a`). Compares floating point values in registers `A` and `B`
- `111 (7)` Double precision comparison (bit pattern `a`). Compares double precision values in registers `A` and `B`

## 7. Branching
```
(a)
0110 AAAA 0000 0000 0000 0000 0000 CCCC
^^^^ ^^^^                          ^^^^
Opc  Branch amount                 Branch condition

(b)
0110 IIII IIII IIII IIII 0000 0000 CCCC
^^^^ ^^^^-^^^^-^^^^-^^^^           ^^^^
Opc  Signed immediate              Branch condition
```

This instruction performs conditional and unconditional branching. The condition used depends on the condition (`C`) bits:
- `0000 (0)` Unconditional (bit pattern `a`). Branches by the value in `A` unconditionally.
- `0001 (1)` Unconditional (bit pattern `b`). Branches by the `immediate` unconditionally.
- `0010 (2)` Greater than (bit pattern `a`). Branches by the value in `A` if the `greater` flag is set.
- `0011 (3)` Greater than (bit pattern `b`). Branches by the `immediate` if the `greater` flag is set.
- `0100 (4)` Equal (bit pattern `a`). Branches by the value in `A` if the `equal` flag is set.
- `0101 (5)` Equal (bit pattern `b`). Branches by the `immediate` if the `equal` flag is set.
- `0110 (6)` Smaller than (bit pattern `a`). Branches by the value in `A` if the `smaller` flag is set.
- `0111 (7)` Smaller than (bit pattern `b`). Branches by the `immediate` if the `smaller` flag is set.
- `1000 (8)` Greater equal (bit pattern `a`). Branches by the value in `A` if the `greater` or `equal` flag is set.
- `1001 (9)` Greater equal (bit pattern `b`). Branches by the `immediate` if the `greater` or `equal` flag is set.
- `1010 (A)` Not equal (bit pattern `a`). Branches by the value in `A` if the `equal` flag is not set.
- `1011 (B)` Not equal (bit pattern `b`). Branches by the `immediate` if the `equal` flag is not set.
- `1100 (C)` Smaller equal (bit pattern `a`). Branches by the value in `A` if the `smaller` or `equal` flag is set.
- `1101 (D)` Smaller equal (bit pattern `b`). Branches by the `immediate` if the `smaller` or `equal` flag is set.
- `1110 (E)` Unassigned. Using this will do nothing.
- `1111 (F)` Unassigned. Using this will do nothing.

## 8. Conversions between integers, floats and doubles
```
(a)
0111 AAAA 0000 0000 0000 0000 0000 0CCC
^^^^ ^^^^                           ^^^
Opc  Reg                            Conversion

(b)
0111 IIII IIII IIII IIII 0000 0000 0CCC
^^^^ ^^^^-^^^^-^^^^-^^^^            ^^^
Opc  Signed immediate               Conversion
```

This instruction converts between diffent types of numbers. The way values are converted depends on the conversion (`C`) bits:
- `000 (0)` Immediate to float (bit pattern `b`). Converts the `immediate` to a float. The result is stored in `A`.
- `001 (1)` Immediate to double (bit pattern `b`). Converts the `immediate` to a double. The result is stored in `A`
- `010 (2)` Int to float (bit pattern `a`). Interprets `A` as an integer and converts it to a float. The result is stored in `A`.
- `011 (3)` Int to double (bit pattern `a`). Interprets `A` as an integer and converts it to a double. The result is stored in `A`.
- `100 (4)` Float to int (bit pattern `a`). Interprets `A` as a float and converts it to an integer. The result is stored in `A`.
- `101 (5)` Float to double (bit pattern `a`). Interprets `A` as a float and converts it to a double. The result is stored in `A`.
- `110 (6)` Double to int (bit pattern `a`). Interprets `A` as a double and converts it to an integer. The result is stored in `A`.
- `111 (7)` Double to float (bit pattern `a`). Interprets `A` as a double and converts it to a float. The result is stored in `A`.

## 9. Floating point arithmetic
```
(a)
          Src reg1
          vvvv
1000 AAAA BBBB CCCC 0000 0000 000O OOOO
^^^^ ^^^^      ^^^^              ^-^^^^
Opc  Dest reg  Src reg2          Operation

(b)
1000 AAAA 0000 0000 0000 0000 000O OOOO
^^^^ ^^^^                        ^-^^^^
Opc  Reg                         Operation

(c)
                          Comparison
                          vvv
1000 AAAA BBBB 0000 0000 0CCC 000O OOOO
^^^^ ^^^^ ^^^^                   ^-^^^^
Opc  Reg1 Reg2                   Operation

(d)
          Src reg
          vvvv
1000 AAAA BBBB 0000 0000 0000 000O OOOO
^^^^ ^^^^                        ^-^^^^
Opc  Dest reg                    Operation
```

This instruction performs an arithmetic operation on floating point numbers. Which one exactly depends on the operation (`O`) bits:
- `00000 ( 0)` Addition (bit pattern `a`). The floats `B` and `C` get added, result is stored in `A`.
- `00001 ( 1)` Subtraction (bit pattern `a`). The float `C` gets subtracted from `B`, result is stored in `A`.
- `00010 ( 2)` Multiplication (bit pattern `a`). The floats `B` and `C` get multiplied, result is stored in `A`.
- `00011 ( 3)` Division (bit pattern `a`). The float `B` gets divided by `C`, result is stored in `A`.
- `00100 ( 4)` Modulo (bit pattern `a`). The float `B` gets modulated by `C`, result is stored in `A`.
- `00101 ( 5)` Negation (bit pattern `b`). The float `A` gets negated, result is stored in `A`.
- `00110 ( 6)` Comparison (bit pattern `c`). Works like the [comparison instruction](#6-comparison), with the `C` bits being the comparison to execute.
- `00111 ( 7)` Power (bit pattern `a`). The float `B` gets raised to the power `C`, result is stored in `A`.
- `01000 ( 8)` Exponent (bit pattern `b`). Calculates the exponential of `B`, result is stored in `A`.
- `01001 ( 9)` Root (bit pattern `a`). Calculates the `C`th root of `B`, result is stored in `A`.
- `01010 ( A)` Square root (bit pattern `d`). Calculates the square root of `B`, result is stored in `A`.
- `01011 ( B)` Cube root (bit pattern `d`). Calculates the cube root of `B`, result is stored in `A`.
- `01100 ( C)` Square (bit pattern `a`). Calculates the square of `B`, result is stored in `A`.
- `01101 ( D)` Cube (bit pattern `a`). Calculates the cube of `B`, result is stored in `A`.
- `01110 ( E)` Logarithm (bit pattern `a`). Calculates the logarithm of `B` to the base `C`, result is stored in `A`.
- `01111 ( F)` Natural logarithm (bit pattern `d`). Calculates the natural logarithm of `B`, result is stored in `A`.
- `10000 (10)` Absolute (bit pattern `d`). Calculates the absolute value OF `B`, result is stored in `A`.
- `10001 (11)` Sine (bit pattern `d`). Calculates the sine of `B`, result is stored in `A`.
- `10010 (12)` Cosine (bit pattern `d`). Calculates the cosine of `B`, result is stored in `A`.
- `10011 (13)` Tangent (bit pattern `d`). Calculates the tangent of `B`, result is stored in `A`.
- `10100 (14)` Inverse sine (bit pattern `d`). Calculates the arcsin of `B`, result is stored in `A`.
- `10101 (15)` Inverse cosine (bit pattern `d`). Calculates the arccos of `B`, result is stored in `A`.
- `10110 (16)` Inverse tangent (bit pattern `d`). Calculates the arctan of `B`, result is stored in `A`.
- `10111 (17)` Floor (bit pattern `d`). Calculates the floor of `B`, result is stored in `A`.
- `11000 (18)` Ceiling (bit pattern `d`). Calculates the ceiling of `B`, result is stored in `A`.
- `11001 (19)` Round (bit pattern `d`). Rounds `B`, result is stored in `A`.
- `11010 (1A)` Minimum (bit pattern `a`). Stores the minimum of `B` and `C` in `A`.
- `11011 (1B)` Maximum (bit pattern `a`). Stores the maximum of `B` and `C` in `A`.
- `11100 (1C)` Sign (bit pattern `d`). Calculates the sign (-1.0, 0.0, 1.0) of `B`, result is stored in `A`. 
- `11101 (1D)` Absolute difference (bit pattern `a`). Calculates the absolute difference between `B` and `C`, result is stored in `A`.
- `11110 (1E)` Load infinity (bit pattern `b`). Loads infinity into `A`.
- `11111 (1F)` Load NaN (bit pattern `b`). Loads NaN into `A`.

## 10. Double precision arithmetic
```
(a)
          Src reg1
          vvvv
1001 AAAA BBBB CCCC 0000 0000 000O OOOO
^^^^ ^^^^      ^^^^              ^-^^^^
Opc  Dest reg  Src reg2          Operation

(b)
1001 AAAA 0000 0000 0000 0000 000O OOOO
^^^^ ^^^^                        ^-^^^^
Opc  Reg                         Operation

(c)
                          Comparison
                          vvv
1001 AAAA BBBB 0000 0000 0CCC 000O OOOO
^^^^ ^^^^ ^^^^                   ^-^^^^
Opc  Reg1 Reg2                   Operation

(d)
          Src reg
          vvvv
1001 AAAA BBBB 0000 0000 0000 000O OOOO
^^^^ ^^^^                        ^-^^^^
Opc  Dest reg                    Operation
```

This instruction performs an arithmetic operation on double precision numbers. Which one exactly depends on the operation (`O`) bits:
- `00000 ( 0)` Addition (bit pattern `a`). The doubles `B` and `C` get added, result is stored in `A`.
- `00001 ( 1)` Subtraction (bit pattern `a`). The double `C` gets subtracted from `B`, result is stored in `A`.
- `00010 ( 2)` Multiplication (bit pattern `a`). The doubles `B` and `C` get multiplied, result is stored in `A`.
- `00011 ( 3)` Division (bit pattern `a`). The double `B` gets divided by `C`, result is stored in `A`.
- `00100 ( 4)` Modulo (bit pattern `a`). The double `B` gets modulated by `C`, result is stored in `A`.
- `00101 ( 5)` Negation (bit pattern `b`). The double `A` gets negated, result is stored in `A`.
- `00110 ( 6)` Comparison (bit pattern `c`). Works like the [comparison instruction](#6-comparison), with the `C` bits being the comparison to execute.
- `00111 ( 7)` Power (bit pattern `a`). The double `B` gets raised to the power `C`, result is stored in `A`.
- `01000 ( 8)` Exponent (bit pattern `b`). Calculates the exponential of `B`, result is stored in `A`.
- `01001 ( 9)` Root (bit pattern `a`). Calculates the `C`th root of `B`, result is stored in `A`.
- `01010 ( A)` Square root (bit pattern `d`). Calculates the square root of `B`, result is stored in `A`.
- `01011 ( B)` Cube root (bit pattern `d`). Calculates the cube root of `B`, result is stored in `A`.
- `01100 ( C)` Square (bit pattern `a`). Calculates the square of `B`, result is stored in `A`.
- `01101 ( D)` Cube (bit pattern `a`). Calculates the cube of `B`, result is stored in `A`.
- `01110 ( E)` Logarithm (bit pattern `a`). Calculates the logarithm of `B` to the base `C`, result is stored in `A`.
- `01111 ( F)` Natural logarithm (bit pattern `d`). Calculates the natural logarithm of `B`, result is stored in `A`.
- `10000 (10)` Absolute (bit pattern `d`). Calculates the absolute value OF `B`, result is stored in `A`.
- `10001 (11)` Sine (bit pattern `d`). Calculates the sine of `B`, result is stored in `A`.
- `10010 (12)` Cosine (bit pattern `d`). Calculates the cosine of `B`, result is stored in `A`.
- `10011 (13)` Tangent (bit pattern `d`). Calculates the tangent of `B`, result is stored in `A`.
- `10100 (14)` Inverse sine (bit pattern `d`). Calculates the arcsin of `B`, result is stored in `A`.
- `10101 (15)` Inverse cosine (bit pattern `d`). Calculates the arccos of `B`, result is stored in `A`.
- `10110 (16)` Inverse tangent (bit pattern `d`). Calculates the arctan of `B`, result is stored in `A`.
- `10111 (17)` Floor (bit pattern `d`). Calculates the floor of `B`, result is stored in `A`.
- `11000 (18)` Ceiling (bit pattern `d`). Calculates the ceiling of `B`, result is stored in `A`.
- `11001 (19)` Round (bit pattern `d`). Rounds `B`, result is stored in `A`.
- `11010 (1A)` Minimum (bit pattern `a`). Stores the minimum of `B` and `C` in `A`.
- `11011 (1B)` Maximum (bit pattern `a`). Stores the maximum of `B` and `C` in `A`.
- `11100 (1C)` Sign (bit pattern `d`). Calculates the sign (-1.0, 0.0, 1.0) of `B`, result is stored in `A`.
- `11101 (1D)` Absolute difference (bit pattern `a`). Calculates the absolute difference between `B` and `C`, result is stored in `A`.
- `11110 (1E)` Load infinity (bit pattern `b`). Loads infinity into `A`.
- `11111 (1F)` Load NaN (bit pattern `b`). Loads NaN into `A`.