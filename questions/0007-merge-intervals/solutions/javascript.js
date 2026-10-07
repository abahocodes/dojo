function merge(intervals) {
  const sorted = [...intervals].sort((a, b) => a[0] - b[0]);
  const merged = [];
  for (const [s, e] of sorted) {
    const top = merged[merged.length - 1];
    if (top && s <= top[1]) top[1] = Math.max(top[1], e);
    else merged.push([s, e]);
  }
  return merged;
}
