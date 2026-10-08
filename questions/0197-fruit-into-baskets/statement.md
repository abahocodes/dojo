A row of trees is described by `fruits`, where `fruits[i]` is the kind of
fruit growing on tree `i`. You carry two baskets, and each basket may hold only
**one kind** of fruit (as many pieces of it as you like).

You choose any tree to start at, then walk to the right, picking exactly one
fruit from every tree you pass, including the first. You must stop as soon as
you reach a tree whose fruit fits in neither basket.

Return the largest number of fruits you can collect. Equivalently, return the
length of the longest contiguous subarray of `fruits` that contains at most two
distinct values.

## Example 1

```
fruits = [3, 1, 3, 2, 2, 1, 1]
output = 4    # start at index 3 and pick [2, 2, 1, 1]
```

## Example 2

```
fruits = [1, 2, 1, 2, 1, 3, 3]
output = 5    # start at index 0 and pick [1, 2, 1, 2, 1]
```

## Constraints

- `1 <= len(fruits) <= 10^5`
- `0 <= fruits[i] < len(fruits)`
