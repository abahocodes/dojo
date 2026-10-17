# Approach: count points, try each diagonal corner

Keep `cnt[(x, y)]`: how many copies of each point are stored. `add` is a
single increment.

For `count(qx, qy)`, the square is determined by its corner diagonally
opposite the query. A stored point `(px, py)` can be that corner when
`|px - qx| == |py - qy|` and the difference is not zero (zero means the same
point, a square of area 0). The two remaining corners are then `(qx, py)` and
`(px, qy)`, and every choice of one copy of each of the three corners is a
different way:

```python
from collections import Counter

class DetectSquares:
    def __init__(self):
        self.cnt = Counter()

    def add(self, point):
        self.cnt[tuple(point)] += 1

    def count(self, point):
        qx, qy = point
        total = 0
        for (px, py), c in self.cnt.items():
            if abs(px - qx) != abs(py - qy) or px == qx:
                continue
            total += c * self.cnt[(qx, py)] * self.cnt[(px, qy)]
        return total
```

(Read missing keys with `.get(..., 0)` in real code: indexing a `Counter`
doesn't insert, but indexing a `defaultdict` would grow it while you iterate.)

A faster variant indexes points by column: for every `y` stored in column
`qx`, the side is `s = y - qy`, and the square lies either to the right
(`px = qx + s`) or to the left (`px = qx - s`). That only visits points that
share the query's column.

## Complexity

- `add`: O(1).
- `count`: O(D) for `D` distinct stored points (or O(points in the query's
  column) with the column index).
- Space: O(D).

## Pitfalls

- Allowing `px == qx` (or `py == qy`): those are degenerate "squares" with
  zero area, and a stored copy of the query point would be counted.
- Adding `1` per square instead of multiplying the three counts, which
  ignores duplicate points.
- Checking only squares to one side, e.g. only `px > qx`; the query can be
  any of the four corners.
- Storing every added point in a list and looping over all triples is
  O(n^3) per query.
