function insertInterval(intervals, newInterval) {
  const result = [];
  let [start, end] = newInterval;
  let i = 0;
  const n = intervals.length;
  // Intervals that end strictly before the new one starts.
  while (i < n && intervals[i][1] < start) result.push(intervals[i++]);
  // Intervals that overlap or touch the new one: absorb them.
  while (i < n && intervals[i][0] <= end) {
    start = Math.min(start, intervals[i][0]);
    end = Math.max(end, intervals[i][1]);
    i++;
  }
  result.push([start, end]);
  // Intervals that start strictly after the merged one ends.
  while (i < n) result.push(intervals[i++]);
  return result;
}
