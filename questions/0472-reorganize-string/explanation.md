# Approach: greedy with a feasibility check

**Feasibility.** A multiset of `r` letters can be arranged with no equal
neighbours exactly when no letter appears more than `ceil(r / 2)` times. Too
many copies of one letter means two of them must touch; otherwise placing
letters in order of frequency into positions `0, 2, 4, ..., 1, 3, 5, ...`
works.

We also need the remaining letters to *start* with something other than the
letter just placed, `c`. That fails only when `c` is forced into the first
slot, which happens when `r` is odd and `c` has exactly `ceil(r / 2)` copies
left. So the rest can follow `c` iff:

- every remaining count is `<= ceil(r / 2)`, and
- the remaining count of `c` is `<= floor(r / 2)`.

**Greedy.** Build the answer left to right. At each position, try letters from
`a` to `z`, skipping the one just placed, and take the first whose placement
keeps the rest feasible. Choosing the smallest feasible letter at each
position is exactly what lexicographic order rewards, and feasibility
guarantees we never get stuck.

```python
def reorganize_string(s):
    counts = [0] * 26
    for ch in s:
        counts[ord(ch) - 97] += 1
    n = len(s)
    if max(counts) > (n + 1) // 2:
        return ""

    result, prev = [], -1
    for pos in range(n):
        rest = n - pos - 1
        for c in range(26):
            if counts[c] == 0 or c == prev:
                continue
            counts[c] -= 1
            if counts[c] <= rest // 2 and max(counts) <= (rest + 1) // 2:
                result.append(chr(97 + c))
                prev = c
                break
            counts[c] += 1
    return "".join(result)
```

**About the heap.** The well-known version of this problem accepts *any* valid
arrangement, and the usual answer repeatedly pops the two most frequent letters
from a max-heap. That produces a valid string but generally not the smallest
one: for `"aabbcc"` it gives `"abcabc"`, while the answer here is `"abacbc"`.
So the greedy must go by letter order and use counts only to stay feasible.

## Complexity

- Time: `O(n · Σ²)` with alphabet size `Σ = 26` (each candidate check scans
  the counts). Tracking the maximum more cleverly gives `O(n · Σ)`, but for
  `n <= 500` this is instant.
- Space: `O(Σ)` besides the output.

## Pitfalls

- Only checking "different from the previous letter": for `"aabbb"` that
  builds `abab` and is left with a lone `b` next to a `b`. The answer is
  `"babab"`. Always check that the rest is still feasible.
- Forgetting the odd case: with `r` odd, a letter holding `ceil(r / 2)` of the
  remaining slots must go next, so it cannot be the letter you just placed.
- Returning a valid but non-minimal arrangement from the two-heap method.
