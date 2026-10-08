# Approach: three largest vs. two smallest times the largest

Think about the sign of the best product. If it uses no negative numbers, or
if all numbers are negative, the three largest values win. If it uses
negatives, it must use exactly two of them (one negative would make the
product non-positive), and those two should have the largest magnitude, i.e.
be the two smallest values, multiplied by the single largest value. So the
answer is

```
max(max1 * max2 * max3, max1 * min1 * min2)
```

Sorting gives these five numbers directly. A single scan that maintains the
top three and bottom two avoids the sort.

```python
def maximum_product_three(nums):
    inf = float("inf")
    max1 = max2 = max3 = -inf
    min1 = min2 = inf
    for x in nums:
        if x >= max1:
            max1, max2, max3 = x, max1, max2
        elif x >= max2:
            max2, max3 = x, max2
        elif x > max3:
            max3 = x
        if x <= min1:
            min1, min2 = x, min1
        elif x < min2:
            min2 = x
    return max(max1 * max2 * max3, max1 * min1 * min2)
```

The extremes are tracked by value, but each update shifts the previous
holders down, so duplicates (e.g. `[5, 5, 5]`) occupy separate slots and the
three chosen elements always sit at distinct positions.

## Complexity

- Time: O(n), one pass (O(n log n) with sorting).
- Space: O(1).

## Pitfalls

- Returning only the product of the three largest. `[-8, -6, 1, 4]` gives 4
  that way instead of 192.
- Using `min1 * min2 * min3`: three negatives make a negative product.
- Using strict comparisons for the largest values and dropping duplicates.
- Sentinels such as `INT_MIN` must never reach a product, or it overflows.
  With at least three elements all five tracked values are real elements,
  and `|product| <= 10^9` fits in a 32-bit int.
