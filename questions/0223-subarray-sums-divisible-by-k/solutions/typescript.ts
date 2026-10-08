function subarraysDivByK(nums: number[], k: number): number {
  const count: number[] = new Array(k).fill(0);
  count[0] = 1;
  let rem = 0;
  let result = 0;
  for (const x of nums) {
    rem = (((rem + x) % k) + k) % k;
    result += count[rem];
    count[rem]++;
  }
  return result;
}
