// 1001. Encode and Decode Strings (medium)
// /show to reread the problem · /test runs visible tests · /submit when done

/**
 * @param {string[]} strs
 * @return {string}
 */
function encode(strs) {

}

/**
 * @param {string} s
 * @return {string[]}
 */
function decode(s) {

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
