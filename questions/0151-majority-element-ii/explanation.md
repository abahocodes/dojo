# Approach: Boyer–Moore vote with two candidates

Three values each occurring more than `n / 3` times would need more than `n`
elements, so at most two values qualify.

Keep two candidate slots, each with a counter. For every element `x`:

- if `x` equals a candidate, increment that candidate's counter;
- otherwise, if a counter is 0, put `x` in that slot with count 1;
- otherwise decrement both counters (this discards one copy of `x` and one
  copy of each candidate: three distinct values).

Every discard removes three distinct values, so it can happen at most `n / 3`
times, and a value with more than `n / 3` copies cannot be fully cancelled:
it ends as one of the candidates. The converse is not true, so a second pass
counts the two candidates and keeps those that clear the bar.

```python
def majority_element(nums):
    c1, c2, n1, n2 = 0, 1, 0, 0
    for x in nums:
        if x == c1:
            n1 += 1
        elif x == c2:
            n2 += 1
        elif n1 == 0:
            c1, n1 = x, 1
        elif n2 == 0:
            c2, n2 = x, 1
        else:
            n1 -= 1
            n2 -= 1
    result = []
    for c in (c1, c2):
        if nums.count(c) > len(nums) // 3 and c not in result:
            result.append(c)
    return result
```

Starting the candidates at two *different* values (0 and 1) keeps them from
ever being equal, so a qualifying value is never reported twice; the
`not in result` check is a cheap extra guard.

## Complexity

- Time: O(n), two passes.
- Space: O(1) extra.

## Pitfalls

- Skipping the verification pass: for `[1, 2, 3]` the vote leaves candidates
  behind even though no value qualifies.
- Checking `x == c1` / `x == c2` *after* the "empty slot" branches: a value
  could then occupy both slots.
- Using `>= n / 3` instead of strictly greater than `floor(n / 3)`.
