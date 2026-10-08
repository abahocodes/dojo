function minMovesToEqual(nums: number[]): number {
  const a = nums.slice();
  const k = a.length >> 1;
  let lo = 0, hi = a.length - 1;
  while (lo < hi) {
    const pivot = a[lo + Math.floor(Math.random() * (hi - lo + 1))];
    // Three-way partition: [lo, lt) < pivot, [lt, gt] == pivot, (gt, hi] > pivot
    let lt = lo, i = lo, gt = hi;
    while (i <= gt) {
      if (a[i] < pivot) {
        [a[lt], a[i]] = [a[i], a[lt]];
        lt++;
        i++;
      } else if (a[i] > pivot) {
        [a[i], a[gt]] = [a[gt], a[i]];
        gt--;
      } else {
        i++;
      }
    }
    if (k < lt) hi = lt - 1;
    else if (k > gt) lo = gt + 1;
    else break;
  }
  const median = a[k];
  let moves = 0;
  for (const v of a) moves += Math.abs(v - median);
  return moves;
}
