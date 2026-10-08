# Hints

## Hint 1
When `left == 1` the head itself moves. A dummy node in front of the list lets
you treat that the same as any other position.

## Hint 2
Find the node just before position `left` — call it `before`. Its successor
will become the *tail* of the reversed run.

## Hint 3
Head insertion: let `tail = before.next`. Repeat `right - left` times: take
`move = tail.next`, unlink it (`tail.next = move.next`), and insert it right
after `before` (`move.next = before.next; before.next = move`).
