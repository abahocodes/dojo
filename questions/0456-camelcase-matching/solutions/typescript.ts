function camelMatch(queries: string[], pattern: string): boolean[] {
  const matches = (query: string): boolean => {
    let j = 0;
    for (const c of query) {
      if (j < pattern.length && c === pattern[j]) j++;
      else if (c >= "A" && c <= "Z") return false;
    }
    return j === pattern.length;
  };
  return queries.map(matches);
}
