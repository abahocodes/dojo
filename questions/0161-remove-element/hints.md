# Hints

## Hint 1
You only need to decide, element by element, whether to keep it.

## Hint 2
Kept elements can be packed at the front of the array as you go, so no
second array is needed.

## Hint 3
Keep a `write` index starting at 0. For every element `x` in order, if
`x != val`, store it at `nums[write]` and increment `write`. Return
`nums[:write]`.
