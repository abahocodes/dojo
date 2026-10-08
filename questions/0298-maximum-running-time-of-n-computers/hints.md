# Hints

## Hint 1
Suppose you want every computer to run for `T` minutes. How much of battery
`i` can actually be used in that time?

## Hint 2
A battery can serve at most one computer per minute, so it contributes at most
`min(batteries[i], T)` minutes. Running `n` computers for `T` minutes needs
`n * T` battery-minutes in total.

## Hint 3
`T` is achievable exactly when `sum(min(b, T)) >= n * T`: batteries with no
more than `T` minutes can be rotated in to fill any holes. That condition is
monotone, so binary search `T` between 0 and `sum(batteries) // n`.
