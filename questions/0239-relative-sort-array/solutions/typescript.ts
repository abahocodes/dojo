function relativeSortArray(arr1: number[], arr2: number[]): number[] {
  const count: number[] = new Array(1001).fill(0);
  for (const x of arr1) count[x]++;
  const result: number[] = [];
  for (const x of arr2) {
    for (let c = 0; c < count[x]; c++) result.push(x);
    count[x] = 0;
  }
  for (let x = 0; x <= 1000; x++) {
    for (let c = 0; c < count[x]; c++) result.push(x);
  }
  return result;
}
