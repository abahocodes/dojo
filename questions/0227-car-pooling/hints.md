# Hints

## Hint 1
The car's load only changes at pickup and drop-off points. What is the load
at a kilometre `x`?

## Hint 2
Instead of adding a trip's passengers to every kilometre it covers, record two
events: `+passengers` at `start` and `-passengers` at `end`.

## Hint 3
Kilometres only go up to 1000, so use an array `change` of size 1001. After
recording every trip, a running sum over `change` gives the load at each
kilometre; if it ever exceeds `capacity`, return `false`.
