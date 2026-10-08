# Hints

## Hint 1
The input is already sorted. Split it into three groups: intervals entirely
to the left of `new_interval`, intervals that meet it, and intervals entirely
to the right.

## Hint 2
An interval is entirely to the left when its `end` is smaller than the new
interval's `start`. It is entirely to the right when its `start` is larger than
the new interval's `end`. Everything else shares at least one point.

## Hint 3
Copy the left group, then fold every meeting interval into the new one by
taking the minimum start and maximum end, append the merged interval, and copy
the right group. One linear pass does it.
