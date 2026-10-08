function evalRpn(tokens: string[]): number {
  const stack: number[] = [];
  for (const t of tokens) {
    if (t === "+" || t === "-" || t === "*" || t === "/") {
      const b = stack.pop()!;
      const a = stack.pop()!;
      if (t === "+") stack.push(a + b);
      else if (t === "-") stack.push(a - b);
      else if (t === "*") stack.push(a * b);
      else stack.push(Math.trunc(a / b));
    } else {
      stack.push(parseInt(t, 10));
    }
  }
  return stack[0];
}
