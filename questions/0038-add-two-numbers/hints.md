# Hints

## Hint 1
Converting the lists to integers is tempting, but the numbers can have
thousands of digits. Think about how you add numbers on paper.

## Hint 2
Because the least significant digit comes first, you can walk both lists
together from the front, adding digit by digit and carrying into the next
position. What if one list is shorter than the other?

## Hint 3
Loop while `l1`, `l2` or `carry` is non-zero. Treat a missing node as `0`, set
`total = a + b + carry`, append a node with `total % 10`, and set
`carry = total // 10`. A dummy head keeps the appending simple.
