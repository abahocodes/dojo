function hammingWeight(n) {
  let count = 0;
  n = n >>> 0;
  while (n !== 0) {
    n = (n & (n - 1)) >>> 0;
    count++;
  }
  return count;
}
