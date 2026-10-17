function subsetsWithDup(nums) {
  nums = [...nums].sort((a, b) => a - b); // equal values become neighbours
  const result = [];
  const current = [];

  function backtrack(start) {
    result.push([...current]);
    for (let i = start; i < nums.length; i++) {
      if (i > start && nums[i] === nums[i - 1]) continue; // already tried here
      current.push(nums[i]);
      backtrack(i + 1);
      current.pop();
    }
  }

  backtrack(0);
  return result;
}
