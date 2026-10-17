# Approach: cycle detection on the digit-square sequence

Let `step(v)` be the sum of the squares of `v`'s digits. Starting from `n`, the
sequence `n, step(n), step(step(n)), ...` is determined entirely by its current
value, so as soon as a value repeats, the sequence cycles forever.

It must repeat: any number with `d` digits maps to at most `81·d`, which is
smaller than the number once `d >= 4`, and below 1000 values stay below 1000.
So the sequence is trapped in a finite range and either hits `1` (a cycle of
length 1) or a cycle that avoids `1`.

That's a linked-list cycle problem, so Floyd's tortoise and hare applies:

```python
def is_happy(n):
    def step(v):
        total = 0
        while v > 0:
            v, d = divmod(v, 10)
            total += d * d
        return total

    slow, fast = n, step(n)
    while fast != 1 and slow != fast:
        slow = step(slow)
        fast = step(step(fast))
    return fast == 1
```

A hash set of visited values is equally correct and arguably simpler: loop
until the value is `1` or already in the set.

## Complexity

- Time: O(log n) for the first step, then a bounded number of steps on small
  values, so effectively O(log n).
- Space: O(1) with two pointers (O(number of distinct values visited) with a
  set, which is also tiny).

## Pitfalls

- Looping "until 1" with no cycle check never terminates on unhappy numbers.
- With Floyd, check `fast == 1` (not `slow`); `fast` gets there first, and once
  at `1` both pointers would meet there anyway.
- Extract digits with `% 10` and integer division; string conversion works but
  is slower and easy to get wrong for the digit values.
