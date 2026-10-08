Houses and heaters stand at integer positions along a straight road. You will
switch every heater on with the **same** warming radius `r`: a heater at
position `h` warms every house whose position `x` satisfies `|x - h| <= r`.

Given the positions of the houses and of the heaters, return the smallest
non-negative integer `r` for which every house is warmed by at least one
heater.

Neither list is sorted, positions may repeat, and a house and a heater may
share a position.

## Example 1

```
houses  = [2, 9, 5]
heaters = [4]
output  = 5    # the house at 9 is 5 away from the only heater
```

## Example 2

```
houses  = [1, 10, 20]
heaters = [20, 1]
output  = 9    # the house at 10 is 9 from the heater at 1 and 10 from the one at 20
```

## Constraints

- `1 <= len(houses), len(heaters) <= 3 * 10^4`
- `1 <= houses[i], heaters[j] <= 10^9`
