# Approach: baseline plus the best fixed-size window

Split the happy customers into two groups:

1. customers of non-grumpy minutes, who are happy regardless: `base`;
2. customers of grumpy minutes that the calm window covers: the window's
   `gain`.

`base` is fixed, so we only need the window of length `minutes` with the
largest gain. Slide it across the day: add `customers[i]` if minute `i` is
grumpy, and subtract the minute that falls out of the window.

```python
def max_satisfied(customers, grumpy, minutes):
    base = sum(c for c, g in zip(customers, grumpy) if g == 0)
    gain = 0
    best = 0
    for i, (c, g) in enumerate(zip(customers, grumpy)):
        if g == 1:
            gain += c
        if i >= minutes and grumpy[i - minutes] == 1:
            gain -= customers[i - minutes]
        best = max(best, gain)
    return base + best
```

Taking the maximum before the first window is full is harmless: a partial
window's gain never exceeds that of the full window containing it, since all
values are non-negative.

## Complexity

- Time: O(n).
- Space: O(1).

## Pitfalls

- Adding all customers inside the window to the gain, which double counts the
  non-grumpy ones already in `base`.
- Recomputing each window's sum from scratch: O(n * minutes).
- Off-by-one when removing the element that leaves the window (it is minute
  `i - minutes`, not `i - minutes + 1`).
