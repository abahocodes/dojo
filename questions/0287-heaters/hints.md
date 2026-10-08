# Hints

## Hint 1
Each house only cares about its closest heater. The answer is the largest of
those "closest heater" distances.

## Hint 2
The closest heater to a house is either the nearest one at or to its left or
the nearest one at or to its right. Sorting the heaters makes both easy to
find.

## Hint 3
Sort `heaters`. For each house, binary search the first heater at position
`>= house`; compare it with the heater just before it, take the smaller
distance, and keep the maximum over all houses.
