A group of people must be ferried across a river. Person `i` weighs
`people[i]`. Every boat can hold **at most two people**, and the total weight
in a boat may not exceed `limit`. Nobody weighs more than `limit`, so everyone
fits in a boat alone.

Return the **minimum number of boats** needed to carry everybody.

## Example 1

```
people = [3, 5, 3, 4]
limit  = 6
output = 3      # [3, 3], [4], [5]
```

## Example 2

```
people = [1, 2]
limit  = 3
output = 1      # both fit in one boat
```

## Constraints

- `1 <= len(people) <= 5 * 10^4`
- `1 <= people[i] <= limit <= 3 * 10^4`
