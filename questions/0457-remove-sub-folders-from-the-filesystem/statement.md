You are given a list of distinct folder paths. Each path starts with `/` and
consists of one or more folder names separated by `/`, such as `"/home/user"`.
A folder is **inside** another folder `P` when its path starts with `P`
followed by `/`: `"/a/b"` is inside `"/a"`, but `"/ab"` is not.

Remove every folder that is inside some other folder in the list, and return
the remaining folders sorted in lexicographic (character code) order.

## Example 1

```
folder = ["/home/user", "/home/user/docs", "/home", "/var/log", "/varlog"]
output = ["/home", "/var/log", "/varlog"]
```

## Example 2

```
folder = ["/x/y/z", "/x/y"]
output = ["/x/y"]
```

## Constraints

- `1 <= len(folder) <= 4 * 10^4`
- `2 <= len(folder[i]) <= 100`
- Paths contain only lowercase English letters and `/`, start with `/`, have
  no empty folder names and no trailing `/`.
- All paths are distinct.
