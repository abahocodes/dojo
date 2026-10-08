function fourSumCount(nums1: number[], nums2: number[], nums3: number[], nums4: number[]): number {
  const sums = new Map<number, number>();
  for (const a of nums1) {
    for (const b of nums2) {
      sums.set(a + b, (sums.get(a + b) ?? 0) + 1);
    }
  }
  let count = 0;
  for (const c of nums3) {
    for (const d of nums4) {
      count += sums.get(-(c + d)) ?? 0;
    }
  }
  return count;
}
