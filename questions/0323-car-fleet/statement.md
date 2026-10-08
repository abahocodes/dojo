`n` cars drive along a one-lane road toward a destination at mile `target`.
Car `i` starts at mile `position[i]` and drives at a constant `speed[i]`
miles per hour. All starting positions are different.

Cars cannot pass each other. When a car catches up with a slower car ahead
of it, it slows down and from then on travels bumper to bumper with it at the
slower car's speed. A group of cars travelling together this way is a
**fleet**; a single car on its own is also a fleet. A car that catches up
with another one exactly at the destination still joins its fleet.

Return the number of fleets that arrive at the destination.

## Example 1

```
target   = 10
position = [0, 4, 2, 7]
speed    = [3, 1, 2, 1]
output   = 2
# the car at 7 arrives alone after 3 hours. The car at 4 needs 6 hours; the
# cars at 2 (4 h) and 0 (3.33 h) catch up with it and arrive together.
```

## Example 2

```
target   = 10
position = [6, 2]
speed    = [1, 2]
output   = 1    # both would need 4 hours, so they meet exactly at mile 10
```

## Constraints

- `1 <= n == len(position) == len(speed) <= 10^5`
- `0 <= position[i] < target <= 10^6`, all `position[i]` are distinct.
- `1 <= speed[i] <= 10^6`
