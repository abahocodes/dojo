# Hints

## Hint 1
Counting the vowels of each length-`k` substring separately costs O(k) per
substring, O(n * k) overall. That is too slow when both are large.

## Hint 2
Two neighbouring substrings differ in only two characters: one leaves on the
left and one enters on the right.

## Hint 3
Count the vowels in `s[0:k]`. Then for each `i` from `k` to `n - 1`, add one if
`s[i]` is a vowel and subtract one if `s[i - k]` is. Track the maximum. You can
stop early if the count ever reaches `k`.
