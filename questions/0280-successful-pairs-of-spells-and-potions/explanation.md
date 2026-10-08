# Approach: sort the potions, binary search per spell

For a spell of strength `s`, a potion `p` succeeds when `s * p >= success`,
i.e. when `p >= need` with `need = ceil(success / s) = (success + s - 1) // s`.
After sorting the potions, the successful ones are exactly a suffix, which
starts at the first index whose value is `>= need` (a lower-bound search).

```python
from bisect import bisect_left

def successful_pairs(spells, potions, success):
    sorted_potions = sorted(potions)
    m = len(sorted_potions)
    result = []
    for s in spells:
        need = (success + s - 1) // s
        result.append(m - bisect_left(sorted_potions, need))
    return result
```

An alternative is to sort the spells too (remembering their original
indices) and sweep a pointer down the potions while the spells get stronger,
in O(n log n + m log m) as well.

## Complexity

- Time: O(m log m + n log m), with `n` spells and `m` potions.
- Space: O(m) for the sorted copy of the potions.

## Pitfalls

- Overflow: `success` and `s * p` can reach `10^10`, so they need 64-bit
  integers in Java, C++ and Go.
- Rounding `need` down: with `success = 8` and `s = 3`, the threshold is `3`,
  not `2`.
- Floating-point division for `need` loses precision in some languages; prefer
  integer ceiling division or compare the product directly.
- Sorting `spells` in place and returning counts in the wrong order. The
  result must follow the original order of `spells`.
