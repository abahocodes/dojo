# Hints

## Hint 1
Only upward steps cost anything. Given a fixed set of climbs you must make,
which climbs should get the ladders?

## Hint 2
Ladders are worth most on the largest climbs. So while walking, you want the
`ladders` biggest climbs seen so far to use ladders and every other climb to
use bricks.

## Hint 3
Push each climb onto a min-heap. Whenever the heap holds more than `ladders`
climbs, pop the smallest and pay for it with bricks. The first time bricks go
negative, you are stuck at the current building.
