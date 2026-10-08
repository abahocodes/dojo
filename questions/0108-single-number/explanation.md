# Approach: XOR everything together

XOR is associative and commutative, every value XORed with itself gives `0`,
and `0` is its identity. So if we XOR the whole list, each pair disappears no
matter where its two copies sit, and what remains is the value with no twin.

```python
def single_number(nums):
    result = 0
    for x in nums:
        result ^= x
    return result
```

Negative numbers are fine: XOR works bit by bit on the two's-complement form,
and the pairs still cancel exactly.

## Complexity

- Time: O(n), one pass.
- Space: O(1).

## Pitfalls

- A hash map or set also works but uses O(n) memory, which misses the point.
- Sorting and scanning pairs works in O(n log n), but it changes the input
  and is easy to get wrong at the last element.
- Start the accumulator at `0`, not at `nums[0]` combined with a loop that
  also includes `nums[0]`.
