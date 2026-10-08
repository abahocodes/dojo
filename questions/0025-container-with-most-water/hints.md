# Hints

## Hint 1
The brute force tries all O(n²) pairs. Start instead with the widest possible
container: the first and last walls. How could a narrower container ever hold
more water?

## Hint 2
Narrowing loses width, so it only helps if the shorter wall gets taller. With
pointers at both ends, which of the two walls is limiting the current
container?

## Hint 3
Always move the pointer at the **shorter** wall inward. Keeping it while
moving the taller one can never give more water: the width shrinks and the
height is still capped by the same short wall. Track the best area as the
pointers meet.
