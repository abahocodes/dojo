# Hints

## Hint 1
A hash map would work, but it costs O(n) memory. The array is sorted: what
does the sum of the smallest and the largest value tell you?

## Hint 2
Put one pointer at each end. If their sum is too small, which pointer can you
move without skipping the answer? If it is too large?

## Hint 3
Too small: advance the left pointer (the left value can't pair with anything
bigger than the current right value). Too large: retreat the right pointer.
Stop when the sum matches and return both positions plus one.
