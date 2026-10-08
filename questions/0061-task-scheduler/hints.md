# Hints

## Hint 1
Only how many jobs each label has matters, not their order in the input. Which
label constrains the schedule the most?

## Hint 2
Let the most frequent label appear `m` times. Its jobs split the timeline into
`m - 1` gaps, and each gap must be at least `n` units long. Other jobs can fill
those gaps instead of idling.

## Hint 3
Lay out `m - 1` blocks of length `n + 1`, then a final block holding one job
of every label that appears `m` times. That gives `(m - 1) * (n + 1) + count_of_max`.
If there are so many jobs that the gaps overflow, no idling is needed and the
answer is just `len(tasks)`. Take the larger of the two.
