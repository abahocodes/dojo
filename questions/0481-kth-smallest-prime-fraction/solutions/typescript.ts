function kthSmallestPrimeFraction(arr: number[], k: number): number[] {
  const n = arr.length;
  let lo = 0;
  let hi = 1;
  for (;;) {
    const mid = (lo + hi) / 2;
    let count = 0;
    let p = 0;
    let q = 1;
    let i = 0;
    for (let j = 1; j < n; j++) {
      while (i < j && arr[i] < mid * arr[j]) i++;
      count += i;
      if (i > 0 && arr[i - 1] * q > p * arr[j]) {
        p = arr[i - 1];
        q = arr[j];
      }
    }
    if (count === k) return [p, q];
    if (count < k) lo = mid;
    else hi = mid;
  }
}
