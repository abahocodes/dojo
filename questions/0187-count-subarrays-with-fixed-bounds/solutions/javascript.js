function countSubarraysFixedBounds(nums, minK, maxK) {
  let total = 0;
  let bad = -1;
  let lastMin = -1;
  let lastMax = -1;
  for (let i = 0; i < nums.length; i++) {
    const v = nums[i];
    if (v < minK || v > maxK) bad = i;
    if (v === minK) lastMin = i;
    if (v === maxK) lastMax = i;
    total += Math.max(0, Math.min(lastMin, lastMax) - bad);
  }
  return total;
}
