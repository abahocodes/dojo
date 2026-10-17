# Hints

## Hint 1
Peel digits off the end of `x` one at a time with `% 10` and `/ 10`, and build
the answer with `rev = rev * 10 + digit`.

## Hint 2
The only risky step is `rev * 10 + digit`. Before doing it, ask: is `rev`
already so large (or so small) that multiplying by 10 must leave the 32-bit
range?

## Hint 3
With `MAX = 2147483647`: if `rev > MAX / 10`, or `rev == MAX / 10` and
`digit > 7`, the next step overflows. Mirror it for negatives with
`MIN = -2147483648` (`rev < MIN / 10`, or `rev == MIN / 10` and `digit < -8`),
or work on the absolute value and check against `MAX` only.
