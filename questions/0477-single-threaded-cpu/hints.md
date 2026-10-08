# Hints

## Hint 1
The CPU only makes decisions at the moments it becomes idle. What do you need
to know at such a moment? The set of tasks that have arrived but not run.

## Hint 2
Sort task indices by enqueue time. Walk through them with a pointer, adding
every task whose enqueue time is `<= now` to a min-heap keyed by
`(processing_time, index)`.

## Hint 3
Each step: if the heap is empty, jump `now` forward to the next enqueue time.
Push every newly available task, pop the top, append its index and add its
processing time to `now`. Keep `now` in 64 bits.
