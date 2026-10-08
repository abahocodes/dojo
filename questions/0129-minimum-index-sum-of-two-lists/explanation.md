# Approach: index map over the first list

Store each string of `list1` with its index in a hash map. Walk `list2` with
index `j`; when `list2[j]` is in the map at index `i`, it is common with sum
`i + j`. Keep the minimum sum and the strings that reach it: a smaller sum
restarts the list, an equal sum extends it.

```python
def find_restaurant(list1, list2):
    index = {s: i for i, s in enumerate(list1)}
    best = None
    result = []
    for j, s in enumerate(list2):
        if s not in index:
            continue
        total = index[s] + j
        if best is None or total < best:
            best = total
            result = [s]
        elif total == best:
            result.append(s)
    return result
```

## Complexity

- Time: O((n + m) * L), where `L` is the string length (hashing).
- Space: O(n * L) for the map over `list1`.

## Pitfalls

- Returning only the first string with the minimum sum when several tie.
- Forgetting to clear the result when a strictly smaller sum shows up.
- Using `best = 0` as the initial minimum; start from "no sum yet" (or
  infinity).
