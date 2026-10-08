# Hints

## Hint 1
You only ever need to look at a node and the one right after it. Can you insert
while you walk, in a single pass?

## Hint 2
The greatest common divisor can be computed with Euclid's algorithm:
`gcd(a, b) = gcd(b, a mod b)`, ending when the second number is `0`.

## Hint 3
Keep `cur = head`. While `cur.next` exists: create a node with
`gcd(cur.val, cur.next.val)`, point it at `cur.next`, point `cur` at it, then
jump `cur` to the original next node (two steps ahead now).
