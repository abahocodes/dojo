# Hints

## Hint 1
Recomputing the maximum of every frame costs O(n · k). When the frame moves, only
one element enters and one leaves. Which elements could still become the maximum
of some future frame?

## Hint 2
If an element is smaller than a newer element to its right, it can never be the
maximum again: the newer one stays in the frame at least as long. Such elements
can be thrown away for good.

## Hint 3
Keep a deque of indices whose values are strictly decreasing from front to back.
For each new index, pop from the back while the back value is `<=` the new one,
then push it. Pop from the front if that index has slid out of the frame. Once
the first frame is complete, the front is the current maximum.
