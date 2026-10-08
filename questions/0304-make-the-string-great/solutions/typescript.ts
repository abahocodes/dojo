function makeGood(s: string): string {
  const stack: string[] = [];
  for (const ch of s) {
    const top = stack[stack.length - 1];
    if (top !== undefined && top !== ch && top.toLowerCase() === ch.toLowerCase()) stack.pop();
    else stack.push(ch);
  }
  return stack.join("");
}
