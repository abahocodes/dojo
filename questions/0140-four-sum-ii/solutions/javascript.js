function fourSumCount(nums1, nums2, nums3, nums4) {
  const sums = new Map();
  for (const a of nums1) {
    for (const b of nums2) {
      sums.set(a + b, (sums.get(a + b) || 0) + 1);
    }
  }
  let count = 0;
  for (const c of nums3) {
    for (const d of nums4) {
      count += sums.get(-(c + d)) || 0;
    }
  }
  return count;
}
