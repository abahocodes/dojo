// 1001. Encode and Decode Strings (medium)
// /show to reread the problem · /test runs visible tests · /submit when done

function encode(strs: string[]): string {

}

function decode(s: string): string[] {

}

// dojo calls this to run the tests; you don't need to change it.
function encodeAndDecode(strs: string[]): string[] {
  const encoded: unknown = encode([...strs]);
  const other: unknown = encode([...strs].reverse().concat(["dojo"]));
  if (typeof encoded !== "string" || typeof other !== "string") {
    return ["<encode must return one string>"];
  }
  return decode(encoded);
}
