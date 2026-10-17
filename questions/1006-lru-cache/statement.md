A cache with room for `capacity` entries has to decide what to throw out when
it is full. The *least recently used* (LRU) policy evicts the entry that has
gone untouched for the longest time. Implement a class `LRUCache` with:

- `LRUCache(capacity)` creates an empty cache holding at most `capacity`
  key-value pairs;
- `get(key)` returns the value stored for `key`, or `-1` if it is not in the
  cache. A successful `get` counts as a use of `key`;
- `put(key, value)` stores `value` under `key`, replacing any old value. This
  also counts as a use of `key`. If the insertion makes the cache hold more
  than `capacity` keys, first remove the least recently used key.

**Both `get` and `put` must run in O(1) average time.**

## How the tests call your code

Below your class the starter code has a small driver,
`lru_cache(capacity, ops, args)`, that dojo calls. It creates one
`LRUCache(capacity)` and walks the lists together: `ops[i]` is `"get"` or
`"put"`, and `args[i]` holds that call's arguments, `[key]` for `get` and
`[key, value]` for `put`. The driver returns the results of the `get` calls,
in order; `put` adds nothing to the output.

## Example 1

```
capacity = 2
ops      = ["put",  "put",  "get", "put",  "get", "get", "get"]
args     = [[1, 10], [2, 20], [1],  [3, 30], [2],   [1],   [3]]
output   = [10, -1, 10, 30]
```

Reading key `1` makes key `2` the least recently used, so putting key `3`
evicts `2`.

## Example 2

```
capacity = 1
ops      = ["put",  "put",  "get", "get", "put",  "get"]
args     = [[4, 1], [4, 2], [4],   [5],   [5, 3], [4]]
output   = [2, -1, -1]
```

Overwriting key `4` doesn't add a second entry. Putting key `5` then evicts
key `4`.

## Constraints

- `1 <= capacity <= 3000`
- `1 <= len(ops) == len(args) <= 2 * 10^4`
- `0 <= key <= 10^4`, `0 <= value <= 10^5`
