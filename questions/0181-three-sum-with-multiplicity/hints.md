# Hints

## Hint 1
The values are tiny (0 to 100) even though the array is long. Instead of
looking at positions, how many times does each value occur?

## Hint 2
Enumerate value triples `x <= y <= z` with `x + y + z == target`. There are
only about 5,000 choices of `(x, y)`, and `z` is then determined. How many
position triples does each value triple stand for?

## Hint 3
With `c[v]` the count of value `v`: all distinct gives `c[x] * c[y] * c[z]`;
`x == y < z` gives `C(c[x], 2) * c[z]`; `x < y == z` gives
`c[x] * C(c[y], 2)`; all equal gives `C(c[x], 3)`. Use 64-bit arithmetic
before taking the modulus.
