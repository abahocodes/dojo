# Approach: three reversals

Let `k = k mod n`. The answer is the last `k` elements followed by the first
`n - k`. Reversing the whole array puts those two blocks in the right place,
but each block is now backwards. Reversing each block on its own restores
their internal order.

```
[1 2 3 4 5 | 6 7]   k = 2
[7 6 | 5 4 3 2 1]   reverse all
[6 7 | 5 4 3 2 1]   reverse first k
[6 7 | 1 2 3 4 5]   reverse the rest
```

```python
def rotate(nums, k):
    n = len(nums)
    k %= n

    def reverse(lo, hi):
        while lo < hi:
            nums[lo], nums[hi] = nums[hi], nums[lo]
            lo += 1
            hi -= 1

    reverse(0, n - 1)
    reverse(0, k - 1)
    reverse(k, n - 1)
    return nums
```

## Complexity

- Time: O(n): every element is swapped at most twice.
- Space: O(1) beyond the input array.

## Pitfalls

- Not reducing `k` modulo `n`: with `k` up to `10^9` a step-by-step rotation
  is far too slow, and index arithmetic like `n - k` goes negative.
- Rotating left instead of right: the last `k` elements move to the front.
- Rotating one position at a time costs O(n * k).
