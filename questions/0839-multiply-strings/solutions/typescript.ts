function multiply(num1: string, num2: string): string {
  // Schoolbook multiplication: digit i of num1 times digit j of num2 lands
  // in positions i + j and i + j + 1 of the product (counting from the left).
  if (num1 === "0" || num2 === "0") return "0";
  const m = num1.length;
  const n = num2.length;
  const out: number[] = new Array(m + n).fill(0);
  for (let i = m - 1; i >= 0; i--) {
    const a = num1.charCodeAt(i) - 48;
    for (let j = n - 1; j >= 0; j--) {
      const total = a * (num2.charCodeAt(j) - 48) + out[i + j + 1];
      out[i + j + 1] = total % 10;
      out[i + j] += Math.floor(total / 10);
    }
  }
  let start = 0;
  while (start < out.length - 1 && out[start] === 0) start++;
  return out.slice(start).join("");
}
