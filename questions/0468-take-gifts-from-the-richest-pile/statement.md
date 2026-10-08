There are several piles of gifts; `gifts[i]` is the number of gifts in pile
`i`. The following happens once per second, for exactly `k` seconds:

1. Pick the pile that currently holds the most gifts. If several piles tie for
   the most, pick any one of them (the result is the same).
2. Take gifts away from that pile until only `floor(sqrt(size))` remain, where
   `size` is the pile's size before this step.

Return the total number of gifts still in all the piles after `k` seconds.

## Example 1

```
gifts  = [30, 8, 50, 2]
k      = 3
output = 16
```

Second 1 shrinks the 50-pile to 7, giving `[30, 8, 7, 2]`. Second 2 shrinks
the 30-pile to 5, giving `[5, 8, 7, 2]`. Second 3 shrinks the 8-pile to 2,
giving `[5, 2, 7, 2]`, which holds 16 gifts in total.

## Example 2

```
gifts  = [1, 1, 1]
k      = 5
output = 3
```

Every pile has size 1 and `floor(sqrt(1)) = 1`, so nothing ever changes.

## Constraints

- `1 <= len(gifts) <= 10^4`
- `1 <= gifts[i] <= 10^9`
- `1 <= k <= 10^4`
- The answer can exceed the 32-bit integer range.
