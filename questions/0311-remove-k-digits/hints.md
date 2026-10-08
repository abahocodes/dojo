# Hints

## Hint 1
Comparing numbers of the same length comes down to the first position where
they differ. Making an early digit smaller matters more than anything later.

## Hint 2
If a digit is larger than the digit right after it, deleting it makes the
number smaller. Deleting a digit that is not larger than its successor never
helps as much.

## Hint 3
Scan left to right with a stack of kept digits. While deletions remain and the
top of the stack is larger than the current digit, pop it (that is one
deletion). If deletions remain at the end, drop them from the tail. Finally
strip leading zeros and map an empty result to `"0"`.
