Build a key-value store that keeps every version of every value. Each write
is stamped with a time, and a read asks what a key held *as of* some time.
Implement a class `TimeMap` with:

- `TimeMap()` creates an empty store;
- `set(key, value, timestamp)` records that `key` held `value` from time
  `timestamp` on;
- `get(key, timestamp)` returns the value of the latest `set` for `key` whose
  time is **at most** `timestamp`, or the empty string `""` if `key` had no
  value by then (or was never set).

The timestamps passed to `set` are strictly increasing across all calls. The
timestamps passed to `get` can be anything in range, including times earlier
than past writes.

## How the tests call your code

Below your class the starter code has a small driver,
`time_map(ops, keys, values, timestamps)`, that dojo calls. It creates one
`TimeMap` and walks the four lists together: `ops[i]` is `"set"` or `"get"`,
and `keys[i]`, `values[i]` and `timestamps[i]` are that call's arguments
(`values[i]` is `""` and unused for `get`). The driver returns the results of
the `get` calls, in order; `set` adds nothing to the output.

## Example 1

```
ops        = ["set",  "get",  "get",  "set",   "get",  "get",  "get"]
keys       = ["mood", "mood", "mood", "mood",  "mood", "mood", "mood"]
values     = ["calm", "",     "",     "happy", "",     "",     ""]
timestamps = [2,      1,      3,      5,       4,      5,      9]
output     = ["", "calm", "calm", "happy", "happy"]
```

Nothing was stored at time `1`. From time `2` to `4` the key holds `"calm"`;
from `5` on it holds `"happy"`.

## Example 2

```
ops        = ["set", "set", "get", "get", "get"]
keys       = ["a",   "b",   "b",   "a",   "c"]
values     = ["x",   "y",   "",    "",    ""]
timestamps = [10,    20,    15,    15,    30]
output     = ["", "x", ""]
```

Keys have separate histories: `"b"` didn't exist yet at time `15`, and `"c"`
was never set.

## Constraints

- `1 <= len(ops) <= 2 * 10^4`, and all four lists have the same length.
- `1 <= len(keys[i]), len(values[i]) <= 100` for `set` (keys and values are
  lowercase letters and digits).
- `1 <= timestamps[i] <= 10^7`
- The timestamps of the `set` calls are strictly increasing.
