function bookCalendarDouble(bookings: number[][]): boolean[] {
  const booked: number[][] = []; // every accepted booking
  const overlaps: number[][] = []; // stretches already covered twice
  const result: boolean[] = [];
  for (const [start, end] of bookings) {
    if (overlaps.some(([s, e]) => Math.max(start, s) < Math.min(end, e))) {
      result.push(false);
      continue;
    }
    for (const [s, e] of booked) {
      const lo = Math.max(start, s);
      const hi = Math.min(end, e);
      if (lo < hi) overlaps.push([lo, hi]);
    }
    booked.push([start, end]);
    result.push(true);
  }
  return result;
}
