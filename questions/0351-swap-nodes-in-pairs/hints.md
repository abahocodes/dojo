# Hints

## Hint 1
The head changes whenever the list has at least two nodes. A dummy node placed
before the head removes that special case.

## Hint 2
To swap a pair `a -> b`, you need to touch three pointers: the node before `a`,
`a.next`, and `b.next`. Draw it out before writing code.

## Hint 3
With `prev` at the node before the pair: let `a = prev.next`, `b = a.next`.
Set `a.next = b.next`, `b.next = a`, `prev.next = b`, then move `prev = a`.
Repeat while `prev.next` and `prev.next.next` both exist.
