# Hints

## Hint 1
The largest element of the result is the larger of the two last elements.
That suggests filling the result from its end.

## Hint 2
Keep one pointer at the end of each input and one write pointer at the end
of the output. Each step copies the larger tail element and moves that
pointer left.

## Hint 3
Loop while `nums2` still has elements. If `nums1` is exhausted, or its tail
is not larger, take from `nums2`. Whatever is left of `nums1` at the end is
already in place (in the in-place version) or just needs copying.
