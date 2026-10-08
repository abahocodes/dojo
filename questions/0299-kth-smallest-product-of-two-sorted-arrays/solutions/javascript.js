function kthSmallestProduct(nums1, nums2, k) {
  const n2 = nums2.length;
  // Number of j with a * nums2[j] <= x.
  const countFor = (a, x) => {
    if (a === 0) return x >= 0 ? n2 : 0;
    let lo = 0;
    let hi = n2;
    if (a > 0) {
      // products increase with j: find the first j with a * nums2[j] > x
      while (lo < hi) {
        const mid = (lo + hi) >> 1;
        if (a * nums2[mid] <= x) lo = mid + 1;
        else hi = mid;
      }
      return lo;
    }
    // products decrease with j: find the first j with a * nums2[j] <= x
    while (lo < hi) {
      const mid = (lo + hi) >> 1;
      if (a * nums2[mid] <= x) hi = mid;
      else lo = mid + 1;
    }
    return n2 - lo;
  };
  const countAtMost = (x) => {
    let total = 0;
    for (const a of nums1) total += countFor(a, x);
    return total;
  };

  const corners = [
    nums1[0] * nums2[0], nums1[0] * nums2[n2 - 1],
    nums1[nums1.length - 1] * nums2[0], nums1[nums1.length - 1] * nums2[n2 - 1],
  ];
  let lo = Math.min(...corners);
  let hi = Math.max(...corners);
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    if (countAtMost(mid) >= k) hi = mid;
    else lo = mid + 1;
  }
  return lo;
}
