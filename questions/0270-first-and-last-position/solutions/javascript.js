function searchRange(nums, target) {
  const firstAtLeast = (x) => {
    let lo = 0;
    let hi = nums.length;
    while (lo < hi) {
      const mid = (lo + hi) >> 1;
      if (nums[mid] < x) lo = mid + 1;
      else hi = mid;
    }
    return lo;
  };
  const first = firstAtLeast(target);
  if (first === nums.length || nums[first] !== target) return [-1, -1];
  return [first, firstAtLeast(target + 1) - 1];
}
