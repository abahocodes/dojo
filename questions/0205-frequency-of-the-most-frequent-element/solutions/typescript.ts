function maxFrequency(nums: number[], k: number): number {
  const a = Int32Array.from(nums).sort();
  let left = 0;
  let window = 0;
  let best = 0;
  for (let right = 0; right < a.length; right++) {
    window += a[right];
    while (a[right] * (right - left + 1) - window > k) window -= a[left++];
    best = Math.max(best, right - left + 1);
  }
  return best;
}
