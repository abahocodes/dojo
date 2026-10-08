# Approach: sliding window with a frequency map

The answer is the longest contiguous subarray with at most two distinct
values. Expand a window to the right, keeping a map from value to how many
times it appears in the window. If adding an element creates a third distinct
value, shrink from the left until one value has disappeared completely. The
window is then the longest valid one ending at `right`.

```python
def total_fruit(fruits):
    count = {}
    left = best = 0
    for right, f in enumerate(fruits):
        count[f] = count.get(f, 0) + 1
        while len(count) > 2:
            g = fruits[left]
            count[g] -= 1
            if count[g] == 0:
                del count[g]
            left += 1
        best = max(best, right - left + 1)
    return best
```

## Complexity

- Time: O(n). Each index enters and leaves the window once.
- Space: O(1). The map never holds more than three keys.

## Pitfalls

- Forgetting to delete a key whose count drops to zero, so the map's size
  overstates how many kinds are in the window.
- Shrinking by only one step: several elements may need to leave before a kind
  disappears.
- Assuming the best window starts where a new kind begins. Use the counts, not
  guesses about run boundaries.
