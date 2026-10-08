You are given an absolute path in a Unix-like file system: a string that
starts with `/` and lists directory names separated by one or more slashes.
Two names are special:

- `.` means "this directory" and has no effect;
- `..` means "the parent directory"; at the root it has no effect, since the
  root has no parent.

Every other name, including ones made only of dots such as `...`, is an
ordinary directory name.

Return the **canonical** form of the path:

- it starts with exactly one `/`;
- consecutive names are separated by exactly one `/`;
- it does not end with `/`, unless it is the root `"/"` itself;
- it contains no `.` or `..` components.

## Example 1

```
path   = "/srv//www/./logs/"
output = "/srv/www/logs"
```

## Example 2

```
path   = "/a/../../b/.../c/.."
output = "/b/..."   # ".." at the root stays at the root; "..." is a real name
```

## Constraints

- `1 <= len(path) <= 3000`
- `path` starts with `/` and consists of English letters, digits, `.`, `_`
  and `/`.
