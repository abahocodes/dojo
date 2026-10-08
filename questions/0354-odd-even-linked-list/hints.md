# Hints

## Hint 1
Imagine pulling the list apart into two chains — one of odd-position nodes and
one of even-position nodes — and then joining them.

## Hint 2
Odd and even nodes alternate, so each odd node's new `next` is the node two
steps ahead, and the same holds for each even node.

## Hint 3
Keep `odd = head`, `even = head.next`, and remember `even_head = even`. While
`even` and `even.next` exist: `odd.next = even.next; odd = odd.next;
even.next = odd.next; even = even.next`. Finally `odd.next = even_head`.
