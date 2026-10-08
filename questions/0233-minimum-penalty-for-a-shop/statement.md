A shop keeps a log of one day split into `n` hours. `customers[i]` is `'Y'`
if customers came during hour `i` and `'N'` if nobody came.

The owner wants to pick a closing hour `j` with `0 <= j <= n`: the shop is
open during hours `0` to `j - 1` and closed from hour `j` onward (`j = 0`
means it never opens, `j = n` means it stays open all day). The **penalty**
of closing at hour `j` is:

- `1` for every open hour (`i < j`) in which nobody came, plus
- `1` for every closed hour (`i >= j`) in which customers came.

Return the closing hour with the smallest penalty. If several hours tie,
return the **earliest** one.

## Example 1

```
customers = "YYNY"
output    = 2
# penalties for j = 0..4 are 3, 2, 1, 2, 1; hours 2 and 4 tie, 2 is earlier
```

## Example 2

```
customers = "YYY"
output    = 3   # stay open all day: penalty 0
```

## Constraints

- `1 <= n <= 10^5`
- Every character of `customers` is `'Y'` or `'N'`.
