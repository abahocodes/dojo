function nextGreaterCircular(nums: number[]): number[] {
  const n = nums.length;
  const result: number[] = new Array(n).fill(-1);
  const stack: number[] = [];
  for (let j = 0; j < 2 * n; j++) {
    const x = nums[j % n];
    while (stack.length > 0 && nums[stack[stack.length - 1]] < x) {
      result[stack.pop()!] = x;
    }
    if (j < n) stack.push(j);
  }
  return result;
}
