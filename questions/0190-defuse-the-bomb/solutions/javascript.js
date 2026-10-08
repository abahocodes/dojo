function decrypt(code, k) {
  const n = code.length;
  const result = new Array(n).fill(0);
  if (k === 0) return result;
  let start = k > 0 ? 1 : n + k;
  let end = k > 0 ? k : n - 1;
  let window = 0;
  for (let j = start; j <= end; j++) window += code[j % n];
  for (let i = 0; i < n; i++) {
    result[i] = window;
    window -= code[start % n];
    start++;
    end++;
    window += code[end % n];
  }
  return result;
}
