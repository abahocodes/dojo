function subArrayRanges(nums: number[]): number {
  const n = nums.length;
  // sign = 1 sums subarray maxima, sign = -1 sums subarray minima.
  const total = (sign: number): number => {
    let result = 0;
    const stack: number[] = [];
    for (let i = 0; i <= n; i++) {
      while (
        stack.length > 0 &&
        (i === n || sign * nums[stack[stack.length - 1]] <= sign * nums[i])
      ) {
        const j = stack.pop()!;
        const left = stack.length > 0 ? stack[stack.length - 1] : -1;
        result += nums[j] * (j - left) * (i - j);
      }
      stack.push(i);
    }
    return result;
  };
  return total(1) - total(-1);
}
