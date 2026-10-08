# Hints

## Hint 1
Folder names never matter for the answer. What does?

## Hint 2
Only how deep you are below main. From depth `d`, the fastest way back is
`d` parent moves.

## Hint 3
Keep a depth counter: `"../"` decreases it (never below 0), `"./"` leaves it
alone, and anything else increases it. Return the final depth.
