function groupStrings(strings) {
  const groups = new Map();
  for (const s of strings) {
    const first = s.charCodeAt(0);
    let key = "";
    for (let i = 0; i < s.length; i++) {
      key += String.fromCharCode(((s.charCodeAt(i) - first + 26) % 26) + 97);
    }
    if (!groups.has(key)) groups.set(key, []);
    groups.get(key).push(s);
  }
  return [...groups.values()];
}
