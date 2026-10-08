function shortestSubarray(nums, k) {
  const n = nums.length;
  const prefix = new Array(n + 1);
  prefix[0] = 0;
  for (let i = 0; i < n; i++) prefix[i + 1] = prefix[i] + nums[i];
  // Deque of indices into prefix, stored in an array with head/tail pointers.
  const dq = new Array(n + 1);
  let head = 0;
  let tail = 0;
  let best = n + 1;
  for (let j = 0; j <= n; j++) {
    while (head < tail && prefix[j] - prefix[dq[head]] >= k) {
      best = Math.min(best, j - dq[head]);
      head++;
    }
    while (head < tail && prefix[dq[tail - 1]] >= prefix[j]) tail--;
    dq[tail++] = j;
  }
  return best <= n ? best : -1;
}
