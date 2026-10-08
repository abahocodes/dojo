function longestConsecutiveSequence(nums) {
  const values = new Set(nums);
  let best = 0;
  for (const x of values) {
    if (!values.has(x - 1)) {
      let length = 1;
      while (values.has(x + length)) length++;
      best = Math.max(best, length);
    }
  }
  return best;
}
