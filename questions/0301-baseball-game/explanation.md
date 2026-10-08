# Approach: a stack of scores

Each operation touches only the most recent scores, so the record is a stack:

- a number: push it;
- `"+"`: push the sum of the top two values (without removing them);
- `"D"`: push twice the top value;
- `"C"`: pop the top value.

The answer is the sum of whatever remains.

```python
def cal_points(operations):
    record = []
    for op in operations:
        if op == "+":
            record.append(record[-1] + record[-2])
        elif op == "D":
            record.append(2 * record[-1])
        elif op == "C":
            record.pop()
        else:
            record.append(int(op))
    return sum(record)
```

## Complexity

- Time: O(n), one constant-time step per operation plus the final sum.
- Space: O(n) for the stack.

## Pitfalls

- `"+"` adds a new score. It does not replace the two scores it reads.
- `"C"` can undo a score that `"+"` or `"D"` created, which exposes older
  scores again. That is why a stack is needed rather than a running total.
- Numbers can be negative. Check for the three symbols first and parse
  everything else, rather than testing whether the first character is a digit.
