function removeOuterParentheses(s: string): string {
  const out: string[] = [];
  let depth = 0;
  for (const ch of s) {
    if (ch === "(") {
      if (depth > 0) out.push(ch);
      depth++;
    } else {
      depth--;
      if (depth > 0) out.push(ch);
    }
  }
  return out.join("");
}
