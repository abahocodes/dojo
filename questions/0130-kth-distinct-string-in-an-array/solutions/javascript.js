function kthDistinct(arr, k) {
  const count = new Map();
  for (const s of arr) count.set(s, (count.get(s) || 0) + 1);
  for (const s of arr) {
    if (count.get(s) === 1 && --k === 0) return s;
  }
  return "";
}
