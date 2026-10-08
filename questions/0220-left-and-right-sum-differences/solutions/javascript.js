function leftRightDifference(nums) {
  let total = 0;
  for (const x of nums) total += x;
  let left = 0;
  const result = [];
  for (const x of nums) {
    const right = total - left - x;
    result.push(Math.abs(left - right));
    left += x;
  }
  return result;
}
