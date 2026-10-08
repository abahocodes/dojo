# Hints

## Hint 1
The number can have ten thousand digits, so converting it to a machine integer
will not work. Work digit by digit, like doubling on paper.

## Hint 2
Doubling a digit produces a carry of at most `1`, and that carry goes to the
digit on its left. Which digits produce a carry?

## Hint 3
A digit `d` sends a carry to its left neighbour exactly when `d >= 5`. So each
new digit is `(2 * d) % 10 + (1 if next digit >= 5 else 0)`, computable in one
left-to-right pass. If the head digit is `>= 5`, put a new node `1` in front.
