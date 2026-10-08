# Approach: set membership

Build a set of jewel kinds once, then walk through the stones and count the
ones whose character is in the set.

```python
def num_jewels_in_stones(jewels, stones):
    kinds = set(jewels)
    return sum(1 for c in stones if c in kinds)
```

Because the alphabet is ASCII letters, the Java, C++ and Go solutions use a
128-entry boolean array instead of a hash set.

## Complexity

- Time: O(J + S) for the lengths of `jewels` and `stones`.
- Space: O(J) for the set (O(1) with a fixed-size array).

## Pitfalls

- Lowercasing the input: case matters, so `'a'` is not a jewel when only
  `'A'` is listed.
- Counting distinct jewel kinds you own instead of individual stones. Every
  stone counts, including repeats.
