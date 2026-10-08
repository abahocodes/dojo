# Approach: monotonic stack of waiting items

Process items from left to right and keep a stack of the indices that have
not yet received a discount. Their prices strictly increase from bottom to
top: any waiting item with a price `>=` a later one would already have been
discounted by it.

At index `j`, every waiting index `i` on top with `prices[i] >= prices[j]`
gets its discount now. `j` is the first qualifying index after `i`, because
`i` was still waiting. Pop it and subtract `prices[j]`. Then push `j`. Items
left on the stack at the end pay full price.

```python
def final_prices(prices):
    result = prices[:]
    stack = []
    for j, p in enumerate(prices):
        while stack and prices[stack[-1]] >= p:
            result[stack.pop()] -= p
        stack.append(j)
    return result
```

## Complexity

- Time: O(n), each index is pushed and popped at most once.
- Space: O(n) for the stack and the result.

## Pitfalls

- The condition is `<=`, not `<`. An equal later price gives a discount, which
  is why `[6, 6]` becomes `[0, 6]`.
- The discount comes from the **first** qualifying item, not the smallest
  one.
- Store indices on the stack, not prices, so you know which result to update.
- The nested scan is O(n^2) and too slow for `10^5` items.
