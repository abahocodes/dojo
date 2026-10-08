A shop is open for `n` minutes. During minute `i`, `customers[i]` people walk
in and leave before the minute ends. The owner's mood is given by `grumpy`:
if `grumpy[i] == 1` the owner is grumpy during minute `i` and every customer of
that minute leaves unhappy; if `grumpy[i] == 0` they all leave happy.

Once during the day, the owner can stay calm for `minutes` consecutive minutes:
throughout that window nobody is treated grumpily, whatever `grumpy` says.

Return the largest possible number of happy customers over the whole day.

## Example 1

```
customers = [2, 0, 4, 1, 3, 5]
grumpy    = [0, 1, 1, 0, 1, 1]
minutes   = 2
output    = 11   # 3 happy anyway (minutes 0 and 3); calming minutes 4-5 adds 8
```

## Example 2

```
customers = [7]
grumpy    = [1]
minutes   = 1
output    = 7
```

## Constraints

- `1 <= minutes <= n <= 2 * 10^4`, where `n = len(customers) == len(grumpy)`
- `0 <= customers[i] <= 1000`
- `grumpy[i]` is `0` or `1`
