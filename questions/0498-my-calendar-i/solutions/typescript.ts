function bookCalendar(bookings: number[][]): boolean[] {
  // Accepted bookings are disjoint, so sorting by start also sorts the ends.
  const starts: number[] = [];
  const ends: number[] = [];
  const result: boolean[] = [];
  for (const [start, end] of bookings) {
    // lo = number of accepted bookings that begin before `end`.
    let lo = 0;
    let hi = starts.length;
    while (lo < hi) {
      const mid = (lo + hi) >> 1;
      if (starts[mid] < end) lo = mid + 1;
      else hi = mid;
    }
    if (lo > 0 && ends[lo - 1] > start) {
      result.push(false);
      continue;
    }
    starts.splice(lo, 0, start);
    ends.splice(lo, 0, end);
    result.push(true);
  }
  return result;
}
