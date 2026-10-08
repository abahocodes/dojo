function minKBitFlips(nums, k) {
  const n = nums.length;
  const ends = new Uint8Array(n + 1);
  let active = 0;
  let flips = 0;
  for (let i = 0; i < n; i++) {
    active ^= ends[i];
    if ((nums[i] ^ active) === 0) {
      if (i + k > n) return -1;
      flips++;
      active ^= 1;
      ends[i + k] ^= 1;
    }
  }
  return flips;
}
