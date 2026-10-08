# Hints

## Hint 1
Only a right-mover with a left-mover somewhere after it can collide. A
left-mover at the very front, or a right-mover at the very end, is safe.

## Hint 2
Process asteroids from left to right and keep the survivors so far. A new
left-mover can only hit right-movers at the end of that list, nearest first.

## Hint 3
Use the survivor list as a stack. For a new negative asteroid, while the top
is positive and smaller in size, pop it. Then: if the top is positive and of
equal size, pop it and drop the new one; if it is positive and larger, drop
the new one; otherwise push the new one.
