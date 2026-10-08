# Hints

## Hint 1
Think of the tuple as two unordered pairs `{a, b}` and `{c, d}` with the
same product. How many ordered tuples does one such pair of pairs produce?

## Hint 2
Swap `a` and `b`, swap `c` and `d`, and swap the two pairs: that is `2 * 2 * 2
= 8` tuples. Because the values are distinct, two different pairs with the
same product cannot share an element.

## Hint 3
Count the product of every unordered pair in a hash map. A product shared by
`m` pairs contributes `8 * m * (m - 1) / 2` tuples. You can add `8 * count`
on the fly before incrementing the count.
