function findCircleNum(isConnected: number[][]): number {
  const n = isConnected.length;
  const parent: number[] = Array.from({ length: n }, (_, i) => i);
  const find = (x: number): number => {
    while (parent[x] !== x) {
      parent[x] = parent[parent[x]];
      x = parent[x];
    }
    return x;
  };
  let provinces = n;
  for (let i = 0; i < n; i++) {
    const row = isConnected[i];
    for (let j = i + 1; j < n; j++) {
      if (row[j] === 1) {
        const ri = find(i);
        const rj = find(j);
        if (ri !== rj) {
          parent[ri] = rj;
          provinces--;
        }
      }
    }
  }
  return provinces;
}
