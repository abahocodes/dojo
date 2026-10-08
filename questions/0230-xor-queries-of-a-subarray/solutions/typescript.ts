function xorQueries(arr: number[], queries: number[][]): number[] {
  const prefix: number[] = new Array(arr.length + 1).fill(0);
  for (let i = 0; i < arr.length; i++) prefix[i + 1] = prefix[i] ^ arr[i];
  return queries.map(([l, r]) => prefix[r + 1] ^ prefix[l]);
}
