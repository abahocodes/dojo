function mergeSorted(nums1, nums2) {
  const m = nums1.length;
  const n = nums2.length;
  const out = nums1.concat(new Array(n).fill(0));
  let i = m - 1;
  let j = n - 1;
  let w = m + n - 1;
  while (j >= 0) {
    if (i >= 0 && out[i] > nums2[j]) {
      out[w--] = out[i--];
    } else {
      out[w--] = nums2[j--];
    }
  }
  return out;
}
