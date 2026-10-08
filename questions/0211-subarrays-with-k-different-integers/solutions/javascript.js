function subarraysWithKDistinct(nums, k) {
  const atMost = (limit) => {
    const count = new Int32Array(nums.length + 1);
    let distinct = 0;
    let left = 0;
    let total = 0;
    for (let right = 0; right < nums.length; right++) {
      if (count[nums[right]]++ === 0) distinct++;
      while (distinct > limit) {
        if (--count[nums[left]] === 0) distinct--;
        left++;
      }
      total += right - left + 1;
    }
    return total;
  };
  return atMost(k) - atMost(k - 1);
}
