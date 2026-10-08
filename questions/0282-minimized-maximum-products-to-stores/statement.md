A chain has `n` stores and `m` product types. `quantities[i]` is the number
of units of product type `i`, and every unit must be shipped to some store.

The rules for shipping:

- each store receives units of **at most one** product type, in any amount
  (a store may receive nothing);
- a product type may be split across several stores.

Let `x` be the largest number of units any single store receives. Return the
minimum possible value of `x`.

## Example 1

```
n          = 5
quantities = [8, 3]
output     = 3   # 8 -> 3, 3, 2 across three stores; 3 -> one store; one store empty
```

## Example 2

```
n          = 4
quantities = [5, 9, 2, 7]
output     = 9   # four types, four stores: every type must go to a single store
```

## Constraints

- `m == len(quantities)`
- `1 <= m <= n <= 10^5`
- `1 <= quantities[i] <= 10^5`
