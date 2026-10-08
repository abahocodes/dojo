function removeCoveredIntervals(intervals: number[][]): number {
  // Start ascending; for equal starts the longer interval comes first.
  const ordered = [...intervals].sort((a, b) => a[0] - b[0] || b[1] - a[1]);
  let remaining = 0;
  let maxEnd = -1;
  for (const [, end] of ordered) {
    if (end > maxEnd) {
      remaining++;
      maxEnd = end;
    }
  }
  return remaining;
}
