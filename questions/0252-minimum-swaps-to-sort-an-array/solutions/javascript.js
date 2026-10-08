function minSwapsToSort(nums) {
  const n = nums.length;
  // order[k] = index of the k-th smallest value
  const order = Array.from({ length: n }, (_, i) => i).sort((a, b) => nums[a] - nums[b]);
  const seen = new Uint8Array(n);
  let swaps = 0;
  for (let i = 0; i < n; i++) {
    let length = 0;
    for (let j = i; !seen[j]; j = order[j]) {
      seen[j] = 1;
      length++;
    }
    if (length > 0) swaps += length - 1;
  }
  return swaps;
}
