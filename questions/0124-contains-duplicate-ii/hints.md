# Hints

## Hint 1
Comparing every pair of positions works but is quadratic. For a given
position, which earlier occurrence of the same value is the only one worth
checking?

## Hint 2
The most recent earlier occurrence is the closest one. If it is too far away,
every older occurrence is even farther.

## Hint 3
Keep a hash map from value to the last index where it appeared. At index `i`,
if `nums[i]` is in the map and `i - last[nums[i]] <= k`, return `true`;
then store `i` as the new last index.
