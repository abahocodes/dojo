function findClosestElements(arr: number[], k: number, x: number): number[] {
  // Binary search for the left edge of the best window arr[left .. left + k - 1].
  let lo = 0;
  let hi = arr.length - k;
  while (lo < hi) {
    const mid = (lo + hi) >> 1;
    if (x - arr[mid] > arr[mid + k] - x) lo = mid + 1;
    else hi = mid;
  }
  return arr.slice(lo, lo + k);
}
