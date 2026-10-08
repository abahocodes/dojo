A string `jewels` lists the kinds of stone that count as jewels: each
character is one kind, and no kind is listed twice. A string `stones` lists
the stones you own, one character per stone. Letters are **case-sensitive**:
`"a"` and `"A"` are different kinds.

Return how many of your stones are jewels.

## Example 1

```
jewels = "xY"
stones = "xxYyyY"
output = 4       # two 'x' and two 'Y'; the 'y' stones are not jewels
```

## Example 2

```
jewels = "q"
stones = "QQQ"
output = 0
```

## Constraints

- `1 <= len(jewels), len(stones) <= 50`
- `jewels` and `stones` contain only English letters (`a`-`z`, `A`-`Z`).
- The characters of `jewels` are distinct.
