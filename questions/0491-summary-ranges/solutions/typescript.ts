function summaryRanges(nums: number[]): string[] {
  const result: string[] = [];
  let i = 0;
  while (i < nums.length) {
    let j = i;
    while (j + 1 < nums.length && nums[j + 1] === nums[j] + 1) j++;
    result.push(i === j ? `${nums[i]}` : `${nums[i]}->${nums[j]}`);
    i = j + 1;
  }
  return result;
}
