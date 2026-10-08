You are given two lists of strings, `list1` and `list2`; neither list
contains the same string twice. For a string that appears in **both** lists,
its **index sum** is its index in `list1` plus its index in `list2`
(0-based).

Return every common string whose index sum is the smallest possible. The
strings may be returned in **any order**. At least one string is common to
both lists.

## Example 1

```
list1  = ["tea", "juice", "soda", "water"]
list2  = ["water", "milk", "soda", "tea"]
output = ["tea", "water"]   # tea: 0 + 3, water: 3 + 0, soda: 2 + 2
```

## Example 2

```
list1  = ["red", "green", "blue"]
list2  = ["green", "blue"]
output = ["green"]          # green: 1 + 0 = 1, blue: 2 + 1 = 3
```

## Constraints

- `1 <= len(list1), len(list2) <= 1000`
- `1 <= len(list1[i]), len(list2[i]) <= 30`
- Strings consist of lowercase English letters and spaces.
- The strings within each list are distinct, and at least one string
  appears in both lists.
