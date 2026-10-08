function reverseParentheses(s: string): string {
  const n = s.length;
  const partner: number[] = new Array(n).fill(0);
  const opens: number[] = [];
  for (let i = 0; i < n; i++) {
    if (s[i] === "(") {
      opens.push(i);
    } else if (s[i] === ")") {
      const j = opens.pop()!;
      partner[i] = j;
      partner[j] = i;
    }
  }
  const out: string[] = [];
  let step = 1;
  for (let i = 0; i < n; i += step) {
    if (s[i] === "(" || s[i] === ")") {
      i = partner[i];
      step = -step;
    } else {
      out.push(s[i]);
    }
  }
  return out.join("");
}
