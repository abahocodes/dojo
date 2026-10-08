# Hints

## Hint 1
Parse left to right. Three kinds of tokens matter: an element name with its
optional count, an opening parenthesis, and a closing parenthesis with its
optional multiplier. A missing number means `1`.

## Hint 2
Parentheses nest, so use a stack of count maps. `'('` starts a fresh map on
top of the stack. An element adds its count to the map on top.

## Hint 3
On `')'`, read the multiplier, pop the top map, multiply each of its counts and
add them into the map that is now on top. At the end, one map remains: sort
its keys and build the answer, omitting counts equal to `1`.
