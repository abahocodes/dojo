function singleNumber(nums) {
  let result = 0;
  for (const x of nums) result ^= x;
  return result;
}
