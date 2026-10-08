You play a jumping game on an integer array `arr`. You pick a starting index
and then make jumps numbered 1, 2, 3, ... Every jump goes strictly to the
right, and its target depends on whether the jump number is odd or even.
From index `i`:

- **Odd-numbered jump** (1st, 3rd, 5th, ...): go to the index `j > i` whose
  value `arr[j]` is the **smallest** value with `arr[j] >= arr[i]`.
- **Even-numbered jump** (2nd, 4th, 6th, ...): go to the index `j > i` whose
  value `arr[j]` is the **largest** value with `arr[j] <= arr[i]`.

If several indices hold that best value, jump to the **smallest** such index.
If no index qualifies, you cannot jump and the game stops.

A starting index is **good** if you can reach the last index (`len(arr) - 1`)
from it after some number of jumps, possibly zero (so the last index itself is
always good). Return the number of good starting indices.

## Example 1

```
arr    = [5, 1, 3, 4, 2]
output = 3
# start 0: odd jump needs a value >= 5 to the right; none, stuck
# start 1: 1 -> 4 (odd: smallest value >= 1 is 2 at index 4), reached
# start 2: 2 -> 3 (odd: 4), then even needs <= 4 to the right: 2 at index 4, reached
# start 3: odd needs >= 4 to the right; none, stuck
# start 4: already at the end
```

## Example 2

```
arr    = [2, 3, 1, 1, 4]
output = 3
# good starts: 1 (3 -> 4), 3 (1 -> 4) and 4
# start 0: 0 -> 1 (3), even: largest <= 3 is 1 at index 2, odd: 1 at index 3,
#          even: nothing <= 1 after index 3, stuck
# start 2: 2 -> 3 (1, smallest index among the 1s), even: none, stuck
```

## Constraints

- `1 <= len(arr) <= 2 * 10^4`
- `0 <= arr[i] < 10^5`
