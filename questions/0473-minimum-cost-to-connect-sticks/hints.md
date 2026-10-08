# Hints

## Hint 1
A stick's length is paid again every time the stick it belongs to takes part
in a fusion. Which sticks should take part in as few fusions as possible?

## Hint 2
Long sticks should be fused late, short ones early. Greedily fusing the two
shortest sticks available is optimal (this is Huffman coding in disguise).

## Hint 3
Put all lengths in a min-heap. While more than one stick remains, pop the two
smallest, add their sum to the cost, and push the sum back. Keep the total
in a 64-bit integer: at the limits it gets close to `2^31`.
