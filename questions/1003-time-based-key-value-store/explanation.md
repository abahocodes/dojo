# Approach: per-key history plus binary search

Keep a hash map from each key to the list of its writes, as
`(timestamp, value)` pairs. Because `set` timestamps strictly increase, an
append keeps each list sorted by time, so there is no need to insert in the
middle or re-sort.

A `get(key, t)` asks for the last write with timestamp `<= t`. In a sorted
list that is a binary search: find the first entry with timestamp `> t`; the
entry just before it is the answer, and if there is none, return `""`.

```python
from bisect import bisect_right
from collections import defaultdict

class TimeMap:
    def __init__(self):
        self.times = defaultdict(list)   # key -> timestamps, increasing
        self.values = defaultdict(list)  # key -> values, same order

    def set(self, key, value, timestamp):
        self.times[key].append(timestamp)
        self.values[key].append(value)

    def get(self, key, timestamp):
        times = self.times.get(key)
        if not times:
            return ""
        i = bisect_right(times, timestamp)  # first time > timestamp
        return self.values[key][i - 1] if i > 0 else ""
```

Keeping times and values in two parallel lists lets the binary search work on
plain integers.

## Complexity

- Time: O(1) per `set`, O(log m) per `get` for a key with `m` writes.
- Space: O(total writes).

## Pitfalls

- Searching for the first timestamp `>= t` instead of the last `<= t` returns
  a write from the future when there is no exact match.
- A `get` earlier than the key's first write must return `""`, not the first
  value.
- A missing key must return `""` too; with a default-dict, don't let the
  lookup create empty entries you then index into.
- Scanning backwards from the end is O(m) per read; that's what the sorted
  order is for.
