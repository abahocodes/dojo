You are preparing a company for its public offering and can run at most `k`
distinct projects before then. You start with capital `w`.

Project `i` can be started only if your current capital is at least
`capital[i]`. Finishing it adds `profits[i]` to your capital (the required
capital is a threshold, not a cost: nothing is spent). Projects are done one
after another, and each project can be done at most once.

Choose the projects and their order to maximize your capital, and return that
final capital.

## Example 1

```
k       = 2
w       = 0
profits = [1, 2, 3]
capital = [0, 1, 1]
output  = 4
```

Only project 0 is affordable at first; it raises capital to 1. Then project 2
(profit 3) is the best affordable choice, ending with 4.

## Example 2

```
k       = 3
w       = 2
profits = [5, 1, 4, 10]
capital = [3, 0, 2, 9]
output  = 21
```

Do project 2 (capital 6), then project 0 (capital 11), then project 3
(capital 21).

## Constraints

- `1 <= k <= 10^5`
- `1 <= len(profits) == len(capital) <= 10^5`
- `0 <= w, profits[i], capital[i] <= 10^9`
- The answer fits in a signed 32-bit integer.
