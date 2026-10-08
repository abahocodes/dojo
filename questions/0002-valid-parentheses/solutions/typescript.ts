function isValid(s: string): boolean {
  const pairs: Record<string, string> = { ")": "(", "]": "[", "}": "{" };
  const stack: string[] = [];
  for (const ch of s) {
    if (ch in pairs) {
      if (stack.length === 0 || stack[stack.length - 1] !== pairs[ch]) return false;
      stack.pop();
    } else {
      stack.push(ch);
    }
  }
  return stack.length === 0;
}
