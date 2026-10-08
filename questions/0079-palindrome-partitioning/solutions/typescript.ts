function partition(s: string): string[][] {
  const n = s.length;
  // pal[i][j] is true when s[i..j] is a palindrome
  const pal: boolean[][] = Array.from({ length: n }, () => new Array<boolean>(n).fill(false));
  for (let i = n - 1; i >= 0; i--) {
    for (let j = i; j < n; j++) {
      if (s[i] === s[j] && (j - i < 2 || pal[i + 1][j - 1])) pal[i][j] = true;
    }
  }

  const result: string[][] = [];
  const current: string[] = [];

  const backtrack = (start: number): void => {
    if (start === n) {
      result.push([...current]);
      return;
    }
    for (let end = start; end < n; end++) {
      if (pal[start][end]) {
        current.push(s.slice(start, end + 1));
        backtrack(end + 1);
        current.pop();
      }
    }
  };

  backtrack(0);
  return result;
}
