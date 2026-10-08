function intersect(nums1: number[], nums2: number[]): number[] {
  const counts: number[] = new Array(1001).fill(0);
  for (const x of nums1) counts[x]++;
  const out: number[] = [];
  for (const x of nums2) {
    if (counts[x] > 0) {
      counts[x]--;
      out.push(x);
    }
  }
  return out;
}
