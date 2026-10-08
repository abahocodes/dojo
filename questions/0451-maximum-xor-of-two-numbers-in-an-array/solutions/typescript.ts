const BITS = 31; // every value is below 2^31

function findMaximumXor(nums: number[]): number {
  // Binary trie in a flat array: child[2 * node + bit] is the child index, 0 = none.
  const child = new Int32Array(2 * (nums.length * BITS + 1));
  let size = 1;
  let best = 0;
  for (const x of nums) {
    let node = 0;
    for (let b = BITS - 1; b >= 0; b--) {
      const slot = 2 * node + ((x >>> b) & 1);
      if (child[slot] === 0) child[slot] = size++;
      node = child[slot];
    }
    // Walk toward the opposite bit wherever possible.
    node = 0;
    let cur = 0;
    for (let b = BITS - 1; b >= 0; b--) {
      const bit = (x >>> b) & 1;
      const want = child[2 * node + (bit ^ 1)];
      if (want !== 0) {
        cur |= 1 << b;
        node = want;
      } else {
        node = child[2 * node + bit];
      }
    }
    if (cur > best) best = cur;
  }
  return best;
}
