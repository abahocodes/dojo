function eraseOverlapIntervals(intervals) {
  const sorted = [...intervals].sort((a, b) => a[1] - b[1]);
  let removed = 0;
  let lastEnd = -Infinity;
  // keeping the interval that ends first leaves the most room for the rest
  for (const [start, end] of sorted) {
    if (start >= lastEnd) {
      lastEnd = end;
    } else {
      removed++;
    }
  }
  return removed;
}
