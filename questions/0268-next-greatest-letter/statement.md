You are given an array `letters` of single lowercase letters sorted in
non-decreasing order (the same letter may repeat), and a single lowercase
letter `target`.

Return the **smallest** letter in `letters` that comes strictly **after**
`target` in the alphabet. The search wraps around: if no letter in `letters`
is greater than `target`, return `letters[0]`.

## Example 1

```
letters = ["b", "e", "e", "k", "q"]
target  = "e"
output  = "k"    # "e" itself does not count; the next larger letter is "k"
```

## Example 2

```
letters = ["d", "h", "m"]
target  = "z"
output  = "d"    # nothing is after "z", so wrap around to the first letter
```

## Constraints

- `2 <= len(letters) <= 10^4`
- every `letters[i]` is one lowercase English letter, and `letters` is sorted
  in non-decreasing order
- `letters` contains at least two different letters
- `target` is one lowercase English letter
