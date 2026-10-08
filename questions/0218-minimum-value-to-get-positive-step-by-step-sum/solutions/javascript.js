function minStartValue(nums) {
  let total = 0;
  let low = 0;
  for (const x of nums) {
    total += x;
    low = Math.min(low, total);
  }
  return 1 - low;
}
