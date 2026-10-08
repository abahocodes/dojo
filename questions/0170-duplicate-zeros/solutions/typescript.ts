function duplicateZeros(arr: number[]): number[] {
  const out = arr.slice();
  const n = out.length;
  // shift = number of zeros strictly before index i: out[i] lands at i + shift.
  let shift = 0;
  for (const v of out) if (v === 0) shift++;
  for (let i = n - 1; i >= 0; i--) {
    if (out[i] === 0) {
      shift--;
      if (i + shift + 1 < n) out[i + shift + 1] = 0;
    }
    if (i + shift < n) out[i + shift] = out[i];
  }
  return out;
}
