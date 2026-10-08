# Approach: two pointers from the ends

Start with `left = 0` and `right = n - 1`, the widest container. Its water is
limited by the shorter wall. Any other container that keeps that shorter wall
is narrower and no taller, so it can never be better: we can discard the
shorter wall and move its pointer inward. Repeat until the pointers meet.

```python
def container_with_most_water(heights):
    left, right = 0, len(heights) - 1
    best = 0
    while left < right:
        width = right - left
        if heights[left] < heights[right]:
            best = max(best, width * heights[left])
            left += 1
        else:
            best = max(best, width * heights[right])
            right -= 1
    return best
```

## Complexity

- Time: O(n): each step moves one pointer inward, so there are `n - 1` steps.
- Space: O(1).

## Pitfalls

- Moving the **taller** wall's pointer can skip the optimum; the argument
  only works for the shorter one.
- On equal heights either pointer may move: any better container must use
  neither of the two walls.
- This is not the trapping-rain-water problem: walls in between don't
  matter, and only two walls hold water.
