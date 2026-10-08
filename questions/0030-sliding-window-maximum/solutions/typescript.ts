function maxSlidingWindow(nums: number[], k: number): number[] {
  // Deque of indices backed by an array with a moving head.
  const dq: number[] = [];
  let head = 0;
  const out: number[] = [];
  for (let i = 0; i < nums.length; i++) {
    const x = nums[i];
    while (dq.length > head && nums[dq[dq.length - 1]] <= x) dq.pop();
    dq.push(i);
    if (dq[head] <= i - k) head++;
    if (i >= k - 1) out.push(nums[dq[head]]);
  }
  return out;
}
