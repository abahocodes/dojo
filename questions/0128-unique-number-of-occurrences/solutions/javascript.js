function uniqueOccurrences(arr) {
  const count = new Map();
  for (const x of arr) count.set(x, (count.get(x) || 0) + 1);
  return new Set(count.values()).size === count.size;
}
