function successfulPairs(spells, potions, success) {
  const sorted = [...potions].sort((a, b) => a - b);
  const m = sorted.length;
  return spells.map((s) => {
    // Smallest potion strength p with s * p >= success.
    const need = Math.ceil(success / s);
    let lo = 0;
    let hi = m;
    while (lo < hi) {
      const mid = (lo + hi) >> 1;
      if (sorted[mid] >= need) hi = mid;
      else lo = mid + 1;
    }
    return m - lo;
  });
}
