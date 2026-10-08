function oddEvenJumps(arr) {
  const n = arr.length;
  // For each index, the first later entry of `order` that lies to its right.
  const targets = (order) => {
    const nxt = new Array(n).fill(-1);
    const stack = [];
    for (const j of order) {
      while (stack.length > 0 && stack[stack.length - 1] < j) nxt[stack.pop()] = j;
      stack.push(j);
    }
    return nxt;
  };
  const idx = Array.from({ length: n }, (_, i) => i);
  const oddNext = targets([...idx].sort((a, b) => arr[a] - arr[b] || a - b));
  const evenNext = targets([...idx].sort((a, b) => arr[b] - arr[a] || a - b));
  const odd = new Array(n).fill(false);
  const even = new Array(n).fill(false);
  odd[n - 1] = even[n - 1] = true;
  let count = 1;
  for (let i = n - 2; i >= 0; i--) {
    if (oddNext[i] !== -1) odd[i] = even[oddNext[i]];
    if (evenNext[i] !== -1) even[i] = odd[evenNext[i]];
    if (odd[i]) count++;
  }
  return count;
}
