# Hints

## Hint 1
Any optimal range can be shrunk until both ends are numbers from the lists.
If you fix the smallest chosen number, which number should you pick from
each other list?

## Hint 2
Pick one pointer per list, starting at the front. The current range runs from
the minimum to the maximum of the pointed-at values. To have any chance of a
narrower range, the minimum must move forward.

## Hint 3
Keep the pointed-at values in a min-heap and track the current maximum
separately. Repeatedly pop the minimum, update the best range, and advance
that list's pointer (pushing the new value and updating the maximum). Stop as
soon as some list runs out.
