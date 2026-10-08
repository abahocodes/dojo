# Hints

## Hint 1
The largest number can never be the smaller element of its pair, so it never
scores. What is the best thing to pair it with?

## Hint 2
Pairing the largest number with the second largest "wastes" as little as
possible: the second largest then scores. Repeat the argument on what remains.

## Hint 3
Sort the list and pair neighbours `(nums[0], nums[1])`, `(nums[2], nums[3])`,
... The answer is the sum of the elements at even indices of the sorted list.
