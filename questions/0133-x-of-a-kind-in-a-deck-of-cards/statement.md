Each card in a deck shows an integer; the deck is given as the array `deck`.
You want to deal the **entire** deck into one or more piles so that:

- every pile has the same number of cards `X`, with `X >= 2`, and
- all cards in one pile show the same value.

Different piles may show the same value. Return `true` if such a split
exists, otherwise `false`.

## Example 1

```
deck   = [5, 9, 5, 9, 9, 5, 7, 7, 7]
output = true    # X = 3: [5,5,5], [9,9,9], [7,7,7]
```

## Example 2

```
deck   = [2, 2, 2, 4, 4]
output = false   # 3 twos and 2 fours share no pile size >= 2
```

## Constraints

- `1 <= len(deck) <= 10^4`
- `0 <= deck[i] < 10^4`
