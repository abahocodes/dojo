# Hints

## Hint 1
Think about how you multiply two numbers by hand on paper: every digit of one
number gets multiplied by every digit of the other, and the partial results are
shifted and added up.

## Hint 2
You don't need to build the partial rows separately. If you index both strings
from the left, the product of `num1[i]` and `num2[j]` contributes to just two
places of the result: position `i + j + 1` (the ones) and position `i + j` (the
carry). An array of `len(num1) + len(num2)` integers is always big enough.

## Hint 3
Loop `i` and `j` from the right. For each pair, add `digit_i * digit_j` to
`out[i + j + 1]`, keep `total % 10` there, and add `total / 10` to
`out[i + j]`. At the end, skip leading zeros (but keep at least one digit) and
join the digits. Handle a `"0"` input up front to keep things simple.
