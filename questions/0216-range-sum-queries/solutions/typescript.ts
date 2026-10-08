function rangeSums(nums: number[], queries: number[][]): number[] {
  const prefix: number[] = new Array(nums.length + 1);
  prefix[0] = 0;
  for (let i = 0; i < nums.length; i++) prefix[i + 1] = prefix[i] + nums[i];
  return queries.map(([l, r]) => prefix[r + 1] - prefix[l]);
}
