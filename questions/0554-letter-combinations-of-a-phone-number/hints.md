# Hints

## Hint 1
Store the keypad as a lookup from digit to letters. The answer has one word
per way of picking a letter for every digit, so it is a product of choices.

## Hint 2
The number of digits isn't fixed, so you can't write one nested loop per
digit. Recursion can play the role of "one loop per digit".

## Hint 3
Write `backtrack(i, prefix)`: if `i == len(digits)`, record `prefix`;
otherwise call `backtrack(i + 1, prefix + letter)` for every letter of
`digits[i]`. Handle the empty input separately: it must give `[]`, not `[""]`.
