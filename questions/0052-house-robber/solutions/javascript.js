function rob(nums) {
  let prev = 0;
  let curr = 0;
  for (const x of nums) {
    const next = Math.max(curr, prev + x);
    prev = curr;
    curr = next;
  }
  return curr;
}
