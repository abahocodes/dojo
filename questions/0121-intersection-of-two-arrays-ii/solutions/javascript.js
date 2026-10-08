function intersect(nums1, nums2) {
  const counts = new Array(1001).fill(0);
  for (const x of nums1) counts[x]++;
  const out = [];
  for (const x of nums2) {
    if (counts[x] > 0) {
      counts[x]--;
      out.push(x);
    }
  }
  return out;
}
