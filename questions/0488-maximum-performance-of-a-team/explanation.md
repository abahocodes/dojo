# Approach: sort by efficiency, keep the fastest k in a min-heap

Every team has some member with the smallest efficiency. Suppose it is
engineer `i`. Then all teammates need `efficiency >= efficiency[i]`, and since
the minimum is already fixed, the best choice is the fastest `k - 1` of those
candidates (more speed only helps).

Sort engineers by efficiency from highest to lowest. When we reach engineer
`i`, every engineer seen earlier has efficiency at least `efficiency[i]`, so
they are exactly the candidates. Keep a min-heap of the speeds chosen so far
together with their sum: push `speed[i]`, and if the heap now holds more than
`k` speeds, pop the slowest. The heap then holds the `k` fastest engineers
among those seen, and `sum * efficiency[i]` is a valid team's performance (if
`i` itself was popped, the remaining team has an even larger minimum
efficiency, so the value is an underestimate of a real team and never too
big). The maximum over all `i` is the answer.

```python
import heapq

MOD = 10**9 + 7

def max_performance(n, speed, efficiency, k):
    engineers = sorted(zip(efficiency, speed), reverse=True)
    heap = []
    total_speed = 0
    best = 0
    for eff, spd in engineers:
        heapq.heappush(heap, spd)
        total_speed += spd
        if len(heap) > k:
            total_speed -= heapq.heappop(heap)
        best = max(best, total_speed * eff)
    return best % MOD
```

## Complexity

- Time: O(n log n) for the sort plus O(n log k) for the heap.
- Space: O(n) for the sorted order, O(k) for the heap.

## Pitfalls

- Taking the modulus inside the loop and comparing reduced values. The
  question asks for the true maximum, reduced afterwards; a reduced value can
  be smaller even when the real value is larger.
- Overflow: the speed sum reaches `10^10` and the product `10^18`. Use 64-bit
  integers in Java, C++ and Go, and `BigInt` (or another exact method) for the
  product in JavaScript, since doubles are exact only up to `2^53`.
- Insisting on exactly `k` engineers. Smaller teams are allowed and can be
  better, which the scan handles by evaluating before the heap is full.
- Sorting by speed instead of efficiency: the heap must be over speeds and
  the sort over efficiencies.
