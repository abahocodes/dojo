# Approach: two cases

**`k = 1`.** Every move takes the first character and puts it last, which is
a rotation by one. The reachable strings are exactly the `n` rotations of `s`,
so return the smallest of them. Comparing all rotations of `s + s` costs
`O(n^2)`, which is fine for `n <= 1000` (Booth's algorithm does it in `O(n)`
if you need it).

**`k >= 2`.** Rotation is still possible (always pick the first character).
On top of that, rotate some character `x` to the front and then repeatedly
move the *second* character to the back: the other characters cycle past `x`
while `x` stays at the front. Stop at any point and rotate the rest of the way
around, and `x` has been lifted out and reinserted at a different place in the
cyclic order. In particular you can swap `x` with its neighbour. Adjacent
swaps generate every permutation, so every arrangement of the letters is
reachable, and the smallest one is the sorted string.

```python
def orderly_queue(s, k):
    if k == 1:
        doubled = s + s
        return min(doubled[i:i + len(s)] for i in range(len(s)))
    return "".join(sorted(s))
```

## Complexity

- Time: `O(n^2)` for `k = 1` (`n` rotations of length `n`), `O(n log n)`
  for `k >= 2` (or `O(n)` with counting sort over 26 letters).
- Space: `O(n)`.

## Pitfalls

- Simulating moves greedily. For `k = 1` choosing the move with the best
  immediate result does nothing useful; you must compare all rotations.
- Assuming `k = 2` only gives a few extra strings. It gives every
  permutation, the same as any larger `k`.
- Returning the smallest rotation for every `k`: `"zebra"`, `k = 2` has the
  sorted answer `"aberz"`, which is not a rotation.
