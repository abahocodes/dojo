# Hints

## Hint 1
Checking all pairs is `O(n^2)`, about five billion pairs at `n = 10^5`. It is
easier to count the good pairs and subtract them from the total.

## Hint 2
Move the terms around: `j - i == nums[j] - nums[i]` is the same as
`nums[i] - i == nums[j] - j`.

## Hint 3
Count the values `nums[i] - i` in a hash map. While scanning, the index `j`
forms a good pair with every earlier index having the same key. The answer
is `n * (n - 1) / 2` minus the good pairs, computed with 64-bit integers.
