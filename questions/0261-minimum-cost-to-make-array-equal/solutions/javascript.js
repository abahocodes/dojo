function minCost(nums, cost) {
  const order = nums.map((_, i) => i).sort((a, b) => nums[a] - nums[b]);
  let total = 0;
  for (const c of cost) total += c;
  let acc = 0;
  let target = nums[order[0]];
  for (const i of order) {
    acc += cost[i];
    if (2 * acc >= total) {
      target = nums[i];
      break;
    }
  }
  let answer = 0;
  for (let i = 0; i < nums.length; i++) answer += cost[i] * Math.abs(nums[i] - target);
  return answer;
}
