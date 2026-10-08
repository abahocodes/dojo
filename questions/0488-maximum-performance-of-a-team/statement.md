You manage `n` engineers numbered `0` to `n - 1`. Engineer `i` works at speed
`speed[i]` and has efficiency `efficiency[i]`. You may assemble a team of
**at least one and at most `k`** engineers. A team's performance is

```
(sum of the team's speeds) * (smallest efficiency in the team)
```

Find the largest performance any team can reach. The true maximum can be as
large as about `10^18`, so return it **modulo `10^9 + 7`**. Choose the team
that maximises the real (unreduced) performance first, and only then take the
remainder.

## Example 1

```
n          = 5
speed      = [3, 8, 2, 6, 4]
efficiency = [7, 2, 9, 5, 6]
k          = 2
output     = 50
```

Engineers 3 and 4 give `(6 + 4) * min(5, 6) = 50`. Adding the fastest
engineer, 1, would drag the minimum efficiency down to 2.

## Example 2

```
n          = 5
speed      = [3, 8, 2, 6, 4]
efficiency = [7, 2, 9, 5, 6]
k          = 3
output     = 65
```

Engineers 0, 3 and 4 give `(3 + 6 + 4) * 5 = 65`.

## Constraints

- `1 <= k <= n <= 10^5`
- `len(speed) == len(efficiency) == n`
- `1 <= speed[i] <= 10^5`
- `1 <= efficiency[i] <= 10^8`
