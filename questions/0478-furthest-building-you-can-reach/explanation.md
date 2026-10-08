# Approach: greedy with a min-heap of ladder climbs

For any prefix of the walk, the cheapest way to get through it is to put
ladders on its `ladders` largest climbs and pay bricks for the rest. So walk
left to right and maintain exactly that assignment incrementally:

- every climb is first tentatively given a ladder (pushed onto a min-heap);
- if that uses more ladders than we own, the smallest ladder climb is switched
  to bricks (popped and subtracted from `bricks`);
- if bricks go negative, the step from `i` to `i + 1` is impossible, so the
  answer is `i`.

```python
import heapq


def furthest_building(heights, bricks, ladders):
    ladder_climbs = []
    for i in range(len(heights) - 1):
        climb = heights[i + 1] - heights[i]
        if climb <= 0:
            continue
        heapq.heappush(ladder_climbs, climb)
        if len(ladder_climbs) > ladders:
            bricks -= heapq.heappop(ladder_climbs)
            if bricks < 0:
                return i
    return len(heights) - 1
```

## Complexity

- Time: O(n log L) where `L = ladders + 1` is the heap size bound.
- Space: O(L).

## Pitfalls

- Spending bricks greedily on each climb as it comes and switching to ladders
  only when bricks run out is wrong: a big early climb can waste many bricks.
- Free steps (equal or lower height) must not enter the heap.
- With `ladders = 0` the heap pops immediately, which is still correct.
- Reaching the last building returns `len(heights) - 1`, including when the
  list has one building.
