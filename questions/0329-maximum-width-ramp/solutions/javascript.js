function maxWidthRamp(nums) {
  const stack = [];
  for (let i = 0; i < nums.length; i++) {
    if (stack.length === 0 || nums[i] < nums[stack[stack.length - 1]]) stack.push(i);
  }
  let best = 0;
  for (let j = nums.length - 1; j >= 0; j--) {
    while (stack.length > 0 && nums[stack[stack.length - 1]] <= nums[j]) {
      best = Math.max(best, j - stack.pop());
    }
  }
  return best;
}
