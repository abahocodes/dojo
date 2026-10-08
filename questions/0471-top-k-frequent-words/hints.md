# Hints

## Hint 1
Start by counting how many times each distinct word appears, using a hash map.

## Hint 2
Now you need to rank the distinct words with a two-part key: count
descending, then the word ascending. Most languages let you sort by a custom
comparator or a tuple key such as `(-count, word)`.

## Hint 3
Sort the distinct words by `(-count, word)` and keep the first `k`. To avoid
sorting everything, keep a heap of size `k` whose top is the *worst* kept word
(lowest count, alphabetically last among ties), then sort the `k` survivors.
