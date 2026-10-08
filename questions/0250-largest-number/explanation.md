# Approach: sort with the concatenation comparator

Put `a` before `b` exactly when the string `a + b` is larger than `b + a`.
This comparison is a valid ordering (it is transitive: it is equivalent to
comparing the infinite repetitions `aaa...` and `bbb...`), and an exchange
argument shows the sorted order is optimal: if two neighbours were out of
order, swapping them would make the whole number larger.

After sorting, the largest piece is first. If it is `"0"`, every piece is
zero, and the answer is `"0"` rather than `"00...0"`.

```python
from functools import cmp_to_key


def largest_number(nums):
    parts = [str(x) for x in nums]
    parts.sort(key=cmp_to_key(lambda a, b: (b + a > a + b) - (b + a < a + b)))
    result = "".join(parts)
    return "0" if result[0] == "0" else result
```

Comparing `a + b` with `b + a` as strings is safe because both have the same
length, so lexicographic order equals numeric order.

## Complexity

- Time: O(n log n * L), where `L <= 10` is the number of digits per value.
- Space: O(n * L) for the strings.

## Pitfalls

- Sorting numerically or by plain string comparison (see the hints).
- Forgetting the all-zero case: `[0, 0]` must give `"0"`, not `"00"`.
- Converting the result back to an integer: up to 1000 digits overflow every
  built-in integer type except Python's.
