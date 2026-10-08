# Approach: split and use a stack

Splitting on `/` turns the path into a list of components, with empty strings
where slashes repeat or sit at the ends. Walk the components keeping a stack of
directory names:

- empty or `.`: ignore;
- `..`: pop, if there is anything to pop;
- anything else: push.

The canonical path is `/` followed by the stack joined with `/`.

```python
def simplify_path(path):
    stack = []
    for part in path.split("/"):
        if part == "" or part == ".":
            continue
        if part == "..":
            if stack:
                stack.pop()
        else:
            stack.append(part)
    return "/" + "/".join(stack)
```

## Complexity

- Time: O(n) for the split, the scan and the join.
- Space: O(n) for the components and the stack.

## Pitfalls

- Treating any name made of dots as special. Only exactly `.` and `..` are;
  `...` or `.hidden` are ordinary names.
- Popping from an empty stack on `..` at the root.
- Returning `""` instead of `"/"` when every directory has been popped.
- Splitting functions that drop empty pieces differ between languages; make
  sure empty pieces are skipped rather than pushed.
