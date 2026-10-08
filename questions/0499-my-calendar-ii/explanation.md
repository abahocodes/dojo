# Approach: track the doubly booked stretches

Maintain two lists:

- `booked`: every accepted booking;
- `overlaps`: intersections of pairs of accepted bookings, i.e. the time that
  is already covered twice.

A request `[start, end)` causes a triple booking exactly when it shares a
moment with some stretch in `overlaps`. Two half-open intervals share a moment
when `max(starts) < min(ends)`. If there is no such stretch, accept the
request. Its intersection with each accepted booking becomes newly doubled
time, so add those to `overlaps`, then add the request to `booked`.

```python
def book_calendar_double(bookings):
    booked, overlaps, result = [], [], []
    for start, end in bookings:
        if any(max(start, s) < min(end, e) for s, e in overlaps):
            result.append(False)
            continue
        for s, e in booked:
            lo, hi = max(start, s), min(end, e)
            if lo < hi:
                overlaps.append((lo, hi))
        booked.append((start, end))
        result.append(True)
    return result
```

`overlaps` may contain stretches that overlap each other; that does no harm,
since the test only asks whether the request meets any of them.

An alternative is a sweep line: keep a sorted map of `+1` at each start and
`-1` at each end, tentatively add the request, and reject if the running sum
ever reaches 3. It is also O(n) per request.

## Complexity

- Time: O(n^2) overall. Each request scans `booked` (at most n entries) and
  `overlaps`. Since no moment is covered three times, `overlaps` itself stays
  pairwise disjoint and holds fewer than 2n stretches.
- Space: O(n).

## Pitfalls

- Treating touching intervals as overlapping. Use strict `<`.
- Adding a rejected request to `booked`, or adding its intersections to
  `overlaps` before deciding whether to accept it.
- Checking only against `booked` and counting overlaps per booking: a request
  can meet two bookings that never overlap each other, which is fine.
