# Hints

## Hint 1
For each day you could scan forward until you find a warmer one, but a long
cooling streak makes that quadratic. Can you answer a day's question at the
moment its warmer day shows up instead?

## Hint 2
Walk the days left to right and keep the days that are still waiting for a warmer
reading. Notice that the waiting days always have non-increasing temperatures.

## Hint 3
Keep a stack of indices. For each new day `i`, while the day on top of the stack
is strictly colder than `temperatures[i]`, pop it and set its answer to
`i - popped`. Then push `i`. Days left on the stack at the end keep answer `0`.
