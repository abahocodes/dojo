# Hints

## Hint 1
Answering each query with its own loop costs O(len(arr)) per query, up to
9 * 10^8 steps. Look for a precomputation that makes every query O(1).

## Hint 2
XOR behaves a lot like addition: it is associative, and every value is its own
inverse (`x ^ x == 0`). Which trick for range *sums* carries over?

## Hint 3
Build `prefix[0] = 0` and `prefix[i + 1] = prefix[i] ^ arr[i]`. Everything
before index `l` appears in both `prefix[r + 1]` and `prefix[l]`, so it
cancels: the answer is `prefix[r + 1] ^ prefix[l]`.
