function exclusiveTime(n: number, logs: string[]): number[] {
  const result: number[] = new Array(n).fill(0);
  const stack: number[] = [];
  let prev = 0;
  for (const entry of logs) {
    const [idText, kind, timeText] = entry.split(":");
    const id = Number(idText);
    const t = Number(timeText);
    if (kind === "start") {
      if (stack.length > 0) result[stack[stack.length - 1]] += t - prev;
      stack.push(id);
      prev = t;
    } else {
      result[stack.pop()!] += t - prev + 1;
      prev = t + 1;
    }
  }
  return result;
}
