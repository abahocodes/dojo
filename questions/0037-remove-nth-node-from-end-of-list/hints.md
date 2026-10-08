# Hints

## Hint 1
If you knew the length `L`, which node (counting from the front) would you
remove? Two passes work. Can you avoid measuring the length first?

## Hint 2
Use two pointers that walk the list with a fixed gap of `n` nodes between them.
When the leading pointer reaches the end, where is the trailing one?

## Hint 3
Start both pointers at a dummy node placed before `head`. Move `fast` ahead
`n` steps, then move both until `fast.next` is `None`. Now `slow.next` is the
node to remove: set `slow.next = slow.next.next` and return `dummy.next`.
