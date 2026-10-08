function minAddToMakeValid(s: string): number {
  let open = 0;
  let added = 0;
  for (const c of s) {
    if (c === "(") open++;
    else if (open > 0) open--;
    else added++;
  }
  return added + open;
}
