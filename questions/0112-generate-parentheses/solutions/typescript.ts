function generateParenthesis(n: number): string[] {
  const result: string[] = [];
  const current: string[] = [];

  function backtrack(open: number, close: number): void {
    if (current.length === 2 * n) {
      result.push(current.join(""));
      return;
    }
    if (open < n) {
      current.push("(");
      backtrack(open + 1, close);
      current.pop();
    }
    if (close < open) {
      current.push(")");
      backtrack(open, close + 1);
      current.pop();
    }
  }

  backtrack(0, 0);
  return result;
}
