A row of tiles is described by `nums`, where every entry is a color code:
`0`, `1` or `2`. Return the tiles rearranged so that all `0`s come first,
then all `1`s, then all `2`s.

Do it in a **single pass** over the array using constant extra space (a
three-way "Dutch national flag" partition). Calling a library sort, or
counting the colors and rewriting the array in a second pass, misses the
point of the exercise.

## Example 1

```
nums   = [2, 0, 1, 2, 0]
output = [0, 0, 1, 2, 2]
```

## Example 2

```
nums   = [2, 1]
output = [1, 2]
```

## Constraints

- `1 <= len(nums) <= 10^5`
- Each `nums[i]` is `0`, `1` or `2`.
