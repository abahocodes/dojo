# Hints

## Hint 1
Every operation only looks at, adds to or removes from the end of the
record. Which data structure works only at one end?

## Hint 2
Keep the record as a stack. `"+"` reads the top two values, `"D"` the top one,
and `"C"` pops.

## Hint 3
Anything that is not `"+"`, `"D"` or `"C"` is a number: parse it (it may be
negative) and push it. Sum the stack at the end.
