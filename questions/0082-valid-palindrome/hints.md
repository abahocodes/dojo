# Hints

## Hint 1
You could build a cleaned-up lowercase copy and compare it with its reverse.
Can you avoid the extra copy?

## Hint 2
Use two pointers, one at each end. Move each pointer inward past any
character that is not a letter or digit.

## Hint 3
When both pointers sit on letters or digits, compare them case-insensitively.
A mismatch means `false`; otherwise move both inward. If the pointers meet or
cross, the text is a palindrome.
