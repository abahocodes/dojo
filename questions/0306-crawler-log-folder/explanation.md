# Approach: track the depth

A full path stack would work (push a name on `"x/"`, pop on `"../"`), but
the names are never used. The minimum number of moves back to main is the
current depth, since every `"../"` climbs exactly one level and nothing climbs
faster. So a counter is enough:

- `"../"`: `depth = max(0, depth - 1)`;
- `"./"`: no change;
- otherwise: `depth += 1`.

```python
def min_operations_to_main(logs):
    depth = 0
    for log in logs:
        if log == "../":
            depth = max(0, depth - 1)
        elif log != "./":
            depth += 1
    return depth
```

## Complexity

- Time: O(n) string comparisons on strings of at most 10 characters.
- Space: O(1).

## Pitfalls

- `"../"` in the main folder does nothing. Letting the depth go negative
  undercounts later moves.
- Test for `"../"` before `"./"` if you compare prefixes or suffixes:
  `"../"` also ends with `"./"`.
- Folder names may contain digits (`"9/"`). Treat anything other than the two
  special operations as entering a folder.
