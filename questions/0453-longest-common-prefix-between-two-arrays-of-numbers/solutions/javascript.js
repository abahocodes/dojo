function longestCommonPrefixNumbers(arr1, arr2) {
  const prefixes = new Set();
  for (let x of arr1) {
    while (x > 0 && !prefixes.has(x)) {
      prefixes.add(x);
      x = Math.floor(x / 10);
    }
  }
  let best = 0;
  for (let y of arr2) {
    while (y > 0 && !prefixes.has(y)) y = Math.floor(y / 10);
    if (y > 0) best = Math.max(best, String(y).length);
  }
  return best;
}
