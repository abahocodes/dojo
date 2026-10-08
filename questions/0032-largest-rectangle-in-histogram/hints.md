# Hints

## Hint 1
The best rectangle's height equals one of the bars' heights. For bar `i` used as
the limiting (shortest) bar, how far can the rectangle stretch left and right?

## Hint 2
Bar `i` can extend until the nearest strictly shorter bar on each side. Finding
"nearest smaller element" for every bar is a classic monotonic-stack task.

## Hint 3
Keep a stack of indices with increasing heights. When bar `i` is shorter than the
top, pop the top: its right limit is `i`, its left limit is the new top (or `-1`
if empty), so its width is `i - new_top - 1`. Add a sentinel height `0` at the
end to flush the stack.
