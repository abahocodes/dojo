function calculate(s: string): number {
  let result = 0, num = 0, sign = 1;
  const stack: number[] = [];
  for (let i = 0; i < s.length; i++) {
    const ch = s[i];
    if (ch >= "0" && ch <= "9") {
      num = num * 10 + (s.charCodeAt(i) - 48);
    } else if (ch === "+" || ch === "-") {
      result += sign * num;
      num = 0;
      sign = ch === "+" ? 1 : -1;
    } else if (ch === "(") {
      stack.push(result, sign);
      result = 0;
      sign = 1;
    } else if (ch === ")") {
      result += sign * num;
      num = 0;
      const savedSign = stack.pop()!;
      const savedResult = stack.pop()!;
      result = savedResult + savedSign * result;
    }
  }
  return result + sign * num;
}
