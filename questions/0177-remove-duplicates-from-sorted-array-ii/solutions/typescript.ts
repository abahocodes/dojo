function removeDuplicatesKeepTwo(nums: number[]): number[] {
  const a = [...nums];
  let k = 0;
  for (let i = 0; i < a.length; i++) {
    const x = a[i];
    if (k < 2 || a[k - 2] !== x) {
      a[k] = x;
      k++;
    }
  }
  return a.slice(0, k);
}
