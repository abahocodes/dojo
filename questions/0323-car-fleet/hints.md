# Hints

## Hint 1
Ignore the interactions for a moment and compute, for each car, the time it
would need to reach `target` alone: `(target - position) / speed`.

## Hint 2
Process cars from the one closest to the destination backwards. A car joins
the fleet in front of it exactly when its solo arrival time is less than or
equal to that fleet's arrival time; joining never changes the fleet's time.

## Hint 3
Sort by position, descending, and keep the arrival time of the most recent
fleet. A car with a strictly larger time starts a new fleet (and becomes the
new reference time). To avoid floating-point trouble, compare
`(target - p1) * s2` with `(target - p2) * s1` using 64-bit integers.
