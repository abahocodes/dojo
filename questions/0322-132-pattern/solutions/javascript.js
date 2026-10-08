function find132Pattern(nums) {
  let third = -Infinity;
  const stack = [];
  for (let i = nums.length - 1; i >= 0; i--) {
    const x = nums[i];
    if (x < third) return true;
    while (stack.length && stack[stack.length - 1] < x) third = stack.pop();
    stack.push(x);
  }
  return false;
}
