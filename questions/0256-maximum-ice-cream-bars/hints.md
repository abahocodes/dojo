# Hints

## Hint 1
To maximize how many bars you buy, which bars should you buy first?

## Hint 2
Always buy the cheapest remaining bar while you can afford it. Swapping any
chosen bar for a cheaper unchosen one never hurts.

## Hint 3
Prices are at most `10^5`, so count how many bars have each price, then walk
prices upward, buying `min(count[p], coins // p)` bars at each price `p`.
