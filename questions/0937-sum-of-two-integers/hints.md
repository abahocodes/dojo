# Hints

## Hint 1
Think about adding two binary numbers by hand, column by column. Which bitwise
operator gives the sum digit of each column if you ignore carries?

## Hint 2
`a ^ b` is the sum without carries. A carry is produced in every column where
both bits are `1`, which is `a & b`, and it lands one column to the left.

## Hint 3
Repeat: `carry = (a & b) << 1`, `a = a ^ b`, `b = carry`, until `b` is `0`;
then `a` is the answer. Each round pushes the carries left, so in 32-bit
arithmetic it ends within 32 rounds. In Python, integers never overflow, so
keep values to 32 bits with a mask (`& 0xFFFFFFFF`) and convert back to a
signed number at the end.
