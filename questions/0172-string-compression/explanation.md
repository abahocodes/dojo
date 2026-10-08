# Approach: two pointers over runs

Let `i` mark the start of the current run and push `j` forward until it hits a
different character (or the end). The run is `chars[i:j]`, of length `j - i`.
Emit the character, then the length if it is greater than 1, and start the
next run at `j`. Every character is visited once by `j`.

```python
def compress(chars):
    out = []
    i, n = 0, len(chars)
    while i < n:
        j = i
        while j < n and chars[j] == chars[i]:
            j += 1
        out.append(chars[i])
        if j - i > 1:
            out.append(str(j - i))
        i = j
    return "".join(out)
```

## Complexity

- Time: O(n), each index is passed once.
- Space: O(n) for the output.

## Pitfalls

- Writing `1` after single characters: `"abc"` stays `"abc"`, not `"a1b1c1"`.
- Writing only one digit for long runs: a run of 12 is `"12"`, not `"1"` or a
  character code.
- Merging non-adjacent equal characters: `"aabaa"` is `"a2ba2"`, not `"a4b"`.
- Forgetting to flush the final run after the loop (if you use the
  "compare with previous character" style).
