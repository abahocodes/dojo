function removeKdigits(num: string, k: number): string {
  const stack: string[] = [];
  for (const d of num) {
    while (k > 0 && stack.length > 0 && stack[stack.length - 1] > d) {
      stack.pop();
      k--;
    }
    stack.push(d);
  }
  stack.length -= k;
  let start = 0;
  while (start < stack.length && stack[start] === "0") start++;
  return start === stack.length ? "0" : stack.slice(start).join("");
}
