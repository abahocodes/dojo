function canPartition(nums: number[]): boolean {
  let total = 0;
  for (const x of nums) total += x;
  if (total % 2 !== 0) return false;
  const half = total / 2;
  // can[s] is 1 when some subset weighs exactly s
  const can = new Uint8Array(half + 1);
  can[0] = 1;
  for (const x of nums) {
    for (let s = half; s >= x; s--) {
      if (can[s - x]) can[s] = 1;
    }
    if (can[half]) return true;
  }
  return can[half] === 1;
}
