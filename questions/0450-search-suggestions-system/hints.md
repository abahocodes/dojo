# Hints

## Hint 1
Sort the products. In a sorted list, where do all the names that share a given
prefix end up relative to each other?

## Hint 2
In sorted order, the names starting with a prefix form one contiguous block,
and the block begins at the first name that is `>=` the prefix. Its first
three names (those that really start with the prefix) are the suggestions.

## Hint 3
Binary-search for the first name `>= prefix` after each typed character. As
the prefix grows, that position never moves left, so you can start each search
from the previous position. Then take up to three names from there, stopping
at the first one that does not start with the prefix.
