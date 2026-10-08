# Approach: greedy walk down a binary trie

Store every number in a binary trie: each level handles one bit, from bit 30
(the highest one a value below `2^31` can have) down to bit 0. For a number
`x`, the best partner is found greedily. At each level, prefer the child whose
bit is the opposite of `x`'s bit, because setting this bit in the XOR is
worth more than every lower bit combined. If that child does not exist, follow
the same bit and leave the XOR bit at 0.

Process the numbers in one pass: insert `x`, then query with `x`. Every pair
is examined when its later element is queried, and `x` itself is already in
the trie, so the walk always succeeds (and `i == j` gives 0).

The trie lives in a flat array, `child[2 * node + bit]`, with 0 meaning "no
child". There are at most `31 · n + 1` nodes.

```python
BITS = 31

def find_maximum_xor(nums):
    child = [0] * (2 * (len(nums) * BITS + 1))
    size = 1
    best = 0
    for x in nums:
        node = 0
        for b in range(BITS - 1, -1, -1):
            slot = 2 * node + ((x >> b) & 1)
            if child[slot] == 0:
                child[slot] = size
                size += 1
            node = child[slot]
        node = 0
        cur = 0
        for b in range(BITS - 1, -1, -1):
            bit = (x >> b) & 1
            want = child[2 * node + (bit ^ 1)]
            if want:
                cur |= 1 << b
                node = want
            else:
                node = child[2 * node + bit]
        best = max(best, cur)
    return best
```

**Alternative without a trie:** build the answer bit by bit. For bit `b` from
high to low, guess that it can be 1, put every number's top bits
(`x >> b`) in a hash set, and check whether two prefixes XOR to the guess.
That is also O(31 · n).

## Complexity

- Time: O(31 · n): one insert and one query per number, each 31 steps.
- Space: O(31 · n) trie nodes.

## Pitfalls

- Start at bit 30, not bit 31 or a smaller bit. Too few levels make values
  collide, and in Java or C++ `1 << 31` overflows a 32-bit `int`.
- Insert before querying (or seed the trie with the first number). Querying
  an empty trie walks into missing children.
- In JavaScript use `>>>` to read bits. With `>>`, a value at or above `2^31`
  would read as negative. Here the values stay below `2^31`, so both work, but
  `>>>` is the safe habit.
- O(n²) pair checking times out at `2 * 10^4` numbers.
