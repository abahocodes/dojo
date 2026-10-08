function frequencySortNumbers(nums) {
  const count = new Map();
  for (const x of nums) count.set(x, (count.get(x) || 0) + 1);
  return [...nums].sort((a, b) => count.get(a) - count.get(b) || b - a);
}
