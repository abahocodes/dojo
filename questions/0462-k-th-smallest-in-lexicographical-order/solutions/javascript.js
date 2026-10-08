function findKthNumber(n, k) {
  // How many numbers in [1, n] start with the decimal digits of prefix.
  const subtreeSize = (prefix) => {
    let count = 0;
    let first = prefix;
    let last = prefix;
    while (first <= n) {
      count += Math.min(n, last) - first + 1;
      first *= 10;
      last = last * 10 + 9;
    }
    return count;
  };

  let current = 1;
  k -= 1;
  while (k > 0) {
    const size = subtreeSize(current);
    if (size <= k) {
      k -= size;
      current += 1;
    } else {
      k -= 1;
      current *= 10;
    }
  }
  return current;
}
