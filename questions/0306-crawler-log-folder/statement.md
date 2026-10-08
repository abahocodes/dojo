A file browser starts in the **main** folder. You are given its log: a list
of operations applied in order. Each one is:

- `"../"`: move to the parent of the current folder. If you are already in the
  main folder, stay there.
- `"./"`: stay in the current folder.
- `"x/"`: move into the child folder named `x`. It always exists.

After all operations, return the minimum number of operations needed to get
back to the main folder.

## Example 1

```
logs   = ["src/", "lib/", "../", "util/", "./"]
output = 2
# main -> src -> src/lib -> src -> src/util -> src/util
# Two "../" operations lead back to main.
```

## Example 2

```
logs   = ["a/", "../", "../", "../"]
output = 0
# Extra "../" operations in main have no effect.
```

## Constraints

- `1 <= len(logs) <= 10^4`
- `2 <= len(logs[i]) <= 10`
- Each `logs[i]` is `"../"`, `"./"`, or a folder name followed by `"/"`.
- Folder names are non-empty and consist of lowercase English letters and
  digits.
