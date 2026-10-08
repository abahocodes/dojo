function minRemoveToMakeValid(s) {
  const chars = s.split("");
  const stack = [];
  for (let i = 0; i < chars.length; i++) {
    if (chars[i] === "(") {
      stack.push(i);
    } else if (chars[i] === ")") {
      if (stack.length) stack.pop();
      else chars[i] = "";
    }
  }
  for (const i of stack) chars[i] = "";
  return chars.join("");
}
