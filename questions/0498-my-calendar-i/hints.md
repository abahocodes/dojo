# Hints

## Hint 1
Two half-open intervals `[s1, e1)` and `[s2, e2)` overlap exactly when
`s1 < e2` and `s2 < e1`. Checking every accepted booking works, but can you do
better?

## Hint 2
Accepted bookings never overlap, so if you keep them sorted by start, their
ends are sorted too. Which single accepted booking can possibly conflict with a
new request?

## Hint 3
Find the accepted booking with the largest start that is still `< end` (a
binary search, or `floorKey`/`lowerEntry` on an ordered map). The request
conflicts exactly when that booking ends after `start`.
