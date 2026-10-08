function longestSquareStreak(nums: number[]): number {
  const present = new Set<number>(nums);
  let best = -1;
  for (const x of present) {
    let length = 1;
    let cur = x;
    while (present.has(cur * cur)) {
      cur *= cur;
      length++;
    }
    if (length >= 2 && length > best) best = length;
  }
  return best;
}
