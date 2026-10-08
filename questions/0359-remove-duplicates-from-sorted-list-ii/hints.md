# Hints

## Hint 1
Because the list is sorted, all copies of a value sit next to each other in a
single run.

## Hint 2
The head itself may need to be removed. A dummy node placed before the head
gives every real node a predecessor you can relink.

## Hint 3
Keep `prev` = the last node known to survive (initially the dummy). If `cur`
starts a run of length > 1, skip the whole run and set `prev.next` to the node
after it; otherwise advance `prev` to `cur`.
