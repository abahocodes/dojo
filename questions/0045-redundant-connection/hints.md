# Hints

## Hint 1
A tree plus one extra edge contains exactly one cycle. Which roads can you
remove to get a tree back?

## Hint 2
Only roads on that cycle are removable. Among those, you want the one listed
last. Imagine adding the roads one by one in the given order: at which moment
does the cycle first appear?

## Hint 3
Add roads in order with union-find. The first road whose two towns are already
connected closes the cycle, and since every other cycle road was added before
it, it is the last one in the list. Return it immediately.
