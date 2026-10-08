# Approach: call stack with a "last credited" cursor

Walk through the log once while mirroring the program's call stack. The key
idea is a cursor `prev`: every unit before `prev` has already been credited to
some function. Each new entry tells us that the units from `prev` up to the
entry belonged to whichever call was on top of the stack.

- `"id:start:t"`: the units `prev .. t - 1` belonged to the call on top (if the
  stack is empty, the CPU was idle). Credit them, push `id`, and set
  `prev = t`.
- `"id:end:t"`: the call on top ran through the **end** of unit `t`, so it gets
  `t - prev + 1` units. Pop it and set `prev = t + 1`.

```python
def exclusive_time(n, logs):
    result = [0] * n
    stack = []
    prev = 0
    for entry in logs:
        fid, kind, ts = entry.split(":")
        fid, t = int(fid), int(ts)
        if kind == "start":
            if stack:
                result[stack[-1]] += t - prev
            stack.append(fid)
            prev = t
        else:
            result[stack.pop()] += t - prev + 1
            prev = t + 1
    return result
```

Recursive calls need no special treatment: each call is its own stack entry,
and all of them credit the same id.

## Complexity

- Time: O(L) for `L` log entries (plus parsing each string).
- Space: O(L) for the stack in the worst case of deep nesting.

## Pitfalls

- Off-by-one: a `start` time marks the **beginning** of a unit, an `end` time
  marks the **end** of one. Forgetting the `+ 1` on `end`, or forgetting to move
  `prev` to `t + 1`, double counts or loses a unit.
- Simulating unit by unit is far too slow: timestamps go up to `10^9`.
- Splitting on ":" and parsing the timestamp as a number — comparing timestamp
  strings lexicographically breaks for different lengths.
