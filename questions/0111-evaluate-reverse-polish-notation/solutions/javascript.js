function evalRpn(tokens) {
  const stack = [];
  for (const t of tokens) {
    if (t === "+" || t === "-" || t === "*" || t === "/") {
      const b = stack.pop();
      const a = stack.pop();
      if (t === "+") stack.push(a + b);
      else if (t === "-") stack.push(a - b);
      else if (t === "*") stack.push(a * b);
      else {
        const q = Math.trunc(a / b);
        stack.push(q === 0 ? 0 : q); // avoid -0
      }
    } else {
      stack.push(Number(t));
    }
  }
  return stack[0];
}
