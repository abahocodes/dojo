function groupStrings(strings: string[]): string[][] {
  const groups = new Map<string, string[]>();
  for (const s of strings) {
    const first = s.charCodeAt(0);
    let key = "";
    for (let i = 0; i < s.length; i++) {
      key += String.fromCharCode(((s.charCodeAt(i) - first + 26) % 26) + 97);
    }
    let group = groups.get(key);
    if (group === undefined) {
      group = [];
      groups.set(key, group);
    }
    group.push(s);
  }
  return [...groups.values()];
}
