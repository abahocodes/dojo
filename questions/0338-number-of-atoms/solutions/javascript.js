function countOfAtoms(formula) {
  const n = formula.length;
  const stack = [new Map()]; // one count map per open group; bottom = whole formula
  let i = 0;
  const isDigit = (c) => c >= "0" && c <= "9";
  const readNumber = () => {
    const start = i;
    while (i < n && isDigit(formula[i])) i++;
    return i > start ? Number(formula.slice(start, i)) : 1;
  };
  while (i < n) {
    const ch = formula[i];
    if (ch === "(") {
      stack.push(new Map());
      i++;
    } else if (ch === ")") {
      i++;
      const mult = readNumber();
      const group = stack.pop();
      const top = stack[stack.length - 1];
      for (const [name, cnt] of group) top.set(name, (top.get(name) || 0) + cnt * mult);
    } else {
      const start = i;
      i++;
      while (i < n && formula[i] >= "a" && formula[i] <= "z") i++;
      const name = formula.slice(start, i);
      const top = stack[stack.length - 1];
      top.set(name, (top.get(name) || 0) + readNumber());
    }
  }
  const counts = stack[0];
  const names = [...counts.keys()].sort();
  let out = "";
  for (const name of names) {
    const cnt = counts.get(name);
    out += cnt > 1 ? name + cnt : name;
  }
  return out;
}
