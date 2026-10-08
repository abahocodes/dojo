A company wants to hire exactly `k` workers from a line of `n` applicants.
`costs[i]` is what applicant `i` would cost.

Hiring happens in `k` rounds. In every round:

- Only the first `candidates` and the last `candidates` applicants **still in
  line** are considered. If fewer than `2 * candidates` remain, every remaining
  applicant is considered (nobody is counted twice).
- The considered applicant with the **lowest cost** is hired; ties go to the
  applicant with the **smallest original index**.
- The hired applicant leaves the line; the others keep their order.

Return the total cost of the `k` hires.

## Example 1

```
costs      = [17, 12, 10, 2, 7, 2, 11, 20, 8]
k          = 3
candidates = 4
output     = 11
```

Round 1 considers `[17, 12, 10, 2]` and `[2, 11, 20, 8]`; the tie at cost 2
goes to index 3. Round 2: the front window becomes `[17, 12, 10, 7]`, and the
2 at index 5 is hired. Round 3: index 4 (cost 7) is hired. Total
`2 + 2 + 7 = 11`.

## Example 2

```
costs      = [1, 2, 4, 1]
k          = 3
candidates = 3
output     = 4
```

All four applicants are visible every round: hire costs 1, 1 and 2.

## Constraints

- `1 <= k, candidates <= len(costs) <= 10^5`
- `1 <= costs[i] <= 10^5`
- The total can exceed the 32-bit range.
