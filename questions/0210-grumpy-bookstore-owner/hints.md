# Hints

## Hint 1
Customers in non-grumpy minutes are happy no matter where the calm window
goes. Count them once and set them aside.

## Hint 2
The calm window only matters for grumpy minutes inside it. Its value is the
sum of `customers[i]` over the grumpy minutes `i` it covers.

## Hint 3
Slide a window of length `minutes` across the day, keeping the sum of grumpy
customers inside it. The answer is the always-happy total plus the best window.
