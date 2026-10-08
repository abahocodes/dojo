# Approach: one pass, counting up-steps and down-steps

Walk over adjacent pairs and keep two counters for the mountain currently
being traced: `up`, the number of rising steps, and `down`, the number of
falling steps after them.

- An equal pair breaks any mountain: reset both counters.
- A rise that comes after some fall (`down > 0`) starts a new mountain: reset
  both counters before counting the rise. The valley element is shared, which
  is what lets mountains like `[0, 2, 1, 3, 1]` overlap at `1`.
- Otherwise a rise increments `up` and a fall increments `down`.

Whenever both counters are positive the current stretch is a valid mountain of
length `up + down + 1`.

```python
def longest_mountain(arr):
    best = up = down = 0
    for i in range(1, len(arr)):
        if arr[i - 1] == arr[i] or (down > 0 and arr[i - 1] < arr[i]):
            up = down = 0
        if arr[i - 1] < arr[i]:
            up += 1
        elif arr[i - 1] > arr[i]:
            down += 1
        if up > 0 and down > 0:
            best = max(best, up + down + 1)
    return best
```

A falling step with `up == 0` (a descent that never climbed) increments
`down`, but such a stretch is never recorded because `up` stays 0 until the
next rise resets everything.

## Complexity

- Time: O(n), a single pass.
- Space: O(1).

## Pitfalls

- Counting a pure ascent or pure descent such as `[1, 2, 3]` as a mountain.
  Both sides need at least one step.
- Treating equal neighbours as part of a slope: `[1, 2, 2, 1]` has no
  mountain.
- Forgetting that consecutive mountains share their valley element:
  `[0, 1, 0, 1, 0]` has mountains of length 3, both using the middle `0`.
- Recording the answer only when a descent ends. A mountain that runs to the
  end of the array must count too, so update the best on every step.
