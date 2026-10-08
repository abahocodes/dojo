function isPerfectSquare(num: number): boolean {
  let lo = 1;
  let hi = Math.min(num, 67108864); // 2^26
  while (lo <= hi) {
    const mid = Math.floor((lo + hi) / 2);
    const square = mid * mid;
    if (square === num) return true;
    if (square < num) lo = mid + 1;
    else hi = mid - 1;
  }
  return false;
}
