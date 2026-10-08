A single-core CPU runs a program made of `n` functions with ids `0` to `n - 1`.
Calls may nest (a function can call another function, or itself), and only the
most recently started call that has not finished is running at any moment.

Time is split into whole units numbered `0, 1, 2, ...`. You are given the
execution trace `logs`, where each entry is a string in one of two forms:

- `"id:start:t"` — a call to function `id` begins at the **start** of unit `t`;
- `"id:end:t"` — the most recent unfinished call (which is to function `id`)
  finishes at the **end** of unit `t`.

The entries are in chronological order and every call is properly nested: each
`end` closes the most recently started open call, and every call is closed.
The CPU may be idle between two top-level calls.

The **exclusive time** of a function is the total number of units during which
one of its calls is the one actually running — time spent inside calls it
made is not included. Return a list of length `n` whose entry `i` is the
exclusive time of function `i`.

## Example 1

```
n = 2
logs = ["0:start:0", "1:start:3", "1:end:5", "0:end:7"]
output = [5, 3]
```

Function 1 runs during units 3, 4 and 5. Function 0 spans units 0 through 7,
which is 8 units, minus the 3 spent in function 1.

## Example 2

```
n = 1
logs = ["0:start:0", "0:start:2", "0:end:4", "0:end:6"]
output = [7]
```

Function 0 calls itself; every unit from 0 to 6 belongs to one of its calls.

## Constraints

- `1 <= n <= 100`
- `2 <= len(logs) <= 500`
- `0 <= id < n`
- `0 <= t <= 10^9`
- The log is valid: timestamps never decrease, an `end` entry's time is at
  least the time of the matching `start`, and calls never overlap except by
  proper nesting.
