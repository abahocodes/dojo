# Hints

## Hint 1
Consider the heaviest person. Whoever shares a boat with them must be light
enough. Who is the best partner to give them?

## Hint 2
If the heaviest person cannot share with the lightest person, they cannot
share with anyone and must go alone. If they can, pairing them with the
lightest never hurts.

## Hint 3
Sort the weights and use two pointers: `i` at the lightest, `j` at the
heaviest. Each step sends one boat with person `j`, plus person `i` if
`people[i] + people[j] <= limit`. Stop when the pointers cross.
