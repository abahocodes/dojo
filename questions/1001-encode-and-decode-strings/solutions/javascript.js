function encode(strs) {
  // Each string becomes "<length>#<string>"; the length says where it ends.
  return strs.map((s) => `${s.length}#${s}`).join("");
}

function decode(s) {
  const result = [];
  let i = 0;
  while (i < s.length) {
    const j = s.indexOf("#", i);
    const length = Number(s.slice(i, j));
    result.push(s.slice(j + 1, j + 1 + length));
    i = j + 1 + length;
  }
  return result;
}

// dojo calls this to run the tests; you don't need to change it.
function encodeAndDecode(strs) {
  const encoded = encode([...strs]);
  const other = encode([...strs].reverse().concat(["dojo"]));
  if (typeof encoded !== "string" || typeof other !== "string") {
    return ["<encode must return one string>"];
  }
  return decode(encoded);
}
