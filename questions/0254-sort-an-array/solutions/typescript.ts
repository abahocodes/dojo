function sortArray(nums: number[]): number[] {
  const n = nums.length;
  let a: number[] = nums.slice();
  let buf: number[] = new Array(n);
  for (let width = 1; width < n; width *= 2) {
    for (let lo = 0; lo < n; lo += 2 * width) {
      const mid = Math.min(lo + width, n);
      const hi = Math.min(lo + 2 * width, n);
      let i = lo, j = mid, k = lo;
      while (i < mid && j < hi) buf[k++] = a[i] <= a[j] ? a[i++] : a[j++];
      while (i < mid) buf[k++] = a[i++];
      while (j < hi) buf[k++] = a[j++];
    }
    [a, buf] = [buf, a];
  }
  return a;
}
