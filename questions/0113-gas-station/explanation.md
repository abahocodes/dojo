# Approach: one greedy pass

Two facts make a single pass enough:

1. If `sum(gas) < sum(cost)`, no start works: the whole loop burns more than
   it collects.
2. If you start at `s` and first run dry leaving station `j`, then no station
   `k` with `s <= k <= j` works either. You reached `k` with a tank of at
   least `0`, so starting fresh at `k` with an empty tank can only be worse.

So scan left to right with a candidate `start` and a `tank`. When the tank goes
negative after station `i`, move the candidate to `i + 1` and empty the tank.
If the total is non-negative, the last candidate is a valid start. Every
earlier index was ruled out by fact 2, so it is also the smallest valid one.

```python
def can_complete_circuit(gas, cost):
    total = tank = start = 0
    for i in range(len(gas)):
        diff = gas[i] - cost[i]
        total += diff
        tank += diff
        if tank < 0:
            start = i + 1
            tank = 0
    return start if total >= 0 else -1
```

Why the final candidate makes it around: the stretch from `start` to the end
never ran dry, and its surplus plus the deficit of the earlier stretch equals
`total >= 0`, so the wrap-around part is covered too.

## Complexity

- Time: O(n), one pass.
- Space: O(1).

## Pitfalls

- Simulating every start is correct but O(n²), too slow for `n = 10^5`.
- The tank may reach exactly `0`; only a negative tank is a failure (`<`, not
  `<=`).
- With several valid starts (for example when every `gas[i] == cost[i]`), the
  answer is the smallest index. The greedy pass gives exactly that.
