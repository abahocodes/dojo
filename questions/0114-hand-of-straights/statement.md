You hold a hand of cards, each showing an integer, given as the list `hand`.
You want to split the **whole** hand into groups so that:

- every group has exactly `group_size` cards, and
- the cards of each group form a run of consecutive values, such as
  `[4, 5, 6]`.

Every card must end up in exactly one group. Return `true` if such a split
exists, otherwise `false`.

## Example 1

```
hand       = [1, 2, 3, 6, 2, 3, 4, 7, 8]
group_size = 3
output     = true      # [1, 2, 3], [2, 3, 4], [6, 7, 8]
```

## Example 2

```
hand       = [1, 2, 3, 4, 5]
group_size = 4
output     = false     # 5 cards can't form groups of 4
```

## Constraints

- `1 <= len(hand) <= 10^4`
- `0 <= hand[i] <= 10^9`
- `1 <= group_size <= len(hand)`
