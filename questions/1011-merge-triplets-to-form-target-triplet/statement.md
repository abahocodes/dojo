You have a list `triplets`, where each `triplets[i] = [a, b, c]` is a triple
of positive integers, and a `target` triple `[x, y, z]`.

You may apply the following merge as many times as you like (including zero
times): pick two different indices `i` and `j`, and replace `triplets[j]`
with the element-wise maximum of the two:

```
triplets[j] = [max(a_i, a_j), max(b_i, b_j), max(c_i, c_j)]
```

Return `True` if, after some sequence of merges, `target` appears as one of
the triplets in the list, and `False` otherwise.

## Example 1

```
triplets = [[3, 4, 2], [1, 6, 1], [2, 4, 5]]
target   = [3, 6, 5]
output   = true
```

Merging the first triplet into the second gives `[3, 6, 2]`; merging the third
into that gives `[3, 6, 5]`.

## Example 2

```
triplets = [[2, 7, 3], [5, 1, 8]]
target   = [5, 7, 3]
output   = false
```

The only triplet with a `5` in front also carries an `8`, and a merge can never
make a value smaller, so using it overshoots the last position. Without it, the
first position never reaches `5`.

## Example 3

```
triplets = [[4, 2, 9], [6, 3, 1], [1, 5, 2], [6, 5, 9]]
target   = [6, 5, 9]
output   = true
```

The target is already in the list; no merge is needed.

## Constraints

- `1 <= len(triplets) <= 10^5`
- `triplets[i].length == target.length == 3`
- `1 <= a, b, c, x, y, z <= 1000`
