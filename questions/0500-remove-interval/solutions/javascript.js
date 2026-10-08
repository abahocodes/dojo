function removeInterval(intervals, toBeRemoved) {
  const [cutLo, cutHi] = toBeRemoved;
  const result = [];
  for (const [a, b] of intervals) {
    if (b <= cutLo || a >= cutHi) {
      result.push([a, b]); // untouched
      continue;
    }
    if (a < cutLo) result.push([a, cutLo]); // piece left of the cut
    if (b > cutHi) result.push([cutHi, b]); // piece right of the cut
  }
  return result;
}
