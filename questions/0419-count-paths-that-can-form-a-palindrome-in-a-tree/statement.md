A tree has `n` nodes numbered `0` to `n - 1` and is rooted at node `0`. You are
given `parent`, where `parent[i]` is the parent of node `i` (`parent[0] = -1`),
and a string `s` of lowercase letters: the edge between `parent[i]` and `i` is
labelled `s[i]`. The character `s[0]` belongs to no edge and is ignored.

Count the pairs of nodes `u < v` such that the letters on the edges of the
path between `u` and `v`, taken in any order, can be rearranged to spell a
palindrome. (A path with zero or one edge always qualifies.)

## Example 1

```
parent = [-1, 0, 0, 1, 1, 2]
s      = "aacabd"
output = 7
```

Node `0` has children `1` (edge `a`) and `2` (edge `c`); node `1` has
children `3` (edge `a`) and `4` (edge `b`); node `2` has child `5` (edge `d`).
The seven qualifying pairs are the five single edges `(0,1)`, `(0,2)`,
`(1,3)`, `(1,4)`, `(2,5)`, plus `(0,3)` with letters `"aa"` and `(2,3)` with
letters `"caa"` (rearranged: `"aca"`).

## Example 2

```
parent = [-1, 0, 1, 2, 3]
s      = "zabba"
output = 8
```

The tree is the path `0 - 1 - 2 - 3 - 4` with edge letters `a, b, b, a`.
Every single edge qualifies (4 pairs), plus `"bb"`, `"abb"`, `"bba"` and
`"abba"`.

## Constraints

- `1 <= n <= 10^5`
- `parent.length == s.length == n`
- `parent[0] == -1`; for `i >= 1`, `0 <= parent[i] < n` and the edges form a
  tree rooted at `0`.
- `s` consists of lowercase English letters.
