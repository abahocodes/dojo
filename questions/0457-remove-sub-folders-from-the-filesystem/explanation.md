# Approach: sort, then compare with the last kept folder

Sort the paths. Every path inside `P` starts with `P + "/"`, and since `/`
sorts before every lowercase letter, all of those paths form a contiguous run
right after `P`. So a single scan suffices: keep the last folder that
survived, drop any path that starts with `last + "/"`, and keep everything
else.

```python
def remove_subfolders(folder):
    result = []
    for path in sorted(folder):
        if not result or not path.startswith(result[-1] + "/"):
            result.append(path)
    return result
```

Alternatives: put the paths in a trie keyed by folder name and output the
topmost marked nodes, or put them in a hash set and, for each path, test every
ancestor obtained by cutting at a `/`.

## Complexity

- Time: O(n log n * L) for the sort, with `L <= 100` the path length; the
  scan is O(n * L).
- Space: O(n * L) for the sorted copy.

## Pitfalls

- Testing `startswith(last)` without the trailing `/`: `"/ab"` is not inside
  `"/a"`.
- Comparing only with the immediately preceding path instead of the last
  *kept* one: in `["/a", "/a/b", "/a/b/c"]` the third path must be compared
  with `"/a"`.
- Returning folders in input order; the output must be sorted.
