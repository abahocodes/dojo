## Hint 1
Flipping the same block twice undoes it, and the order of flips does not
matter. So the question is only *which* starting positions get flipped once.

## Hint 2
Look at the leftmost element. Only a block starting at index 0 can change it,
so whether that block is flipped is forced. Then the next index is forced, and
so on: scan left to right and flip whenever the current bit is still 0.

## Hint 3
Actually inverting `k` elements per flip is `O(n * k)`. Instead track the
parity of flips that currently cover index `i`: when you start a flip at `i`,
toggle the parity and mark index `i + k` as the place where that flip stops
covering. A flip that would run past the end means the answer is `-1`.
