const KEYS = {
  2: "abc", 3: "def", 4: "ghi", 5: "jkl",
  6: "mno", 7: "pqrs", 8: "tuv", 9: "wxyz",
};

function letterCombinations(digits) {
  if (digits.length === 0) return [];
  const result = [];

  function backtrack(i, prefix) {
    if (i === digits.length) {
      result.push(prefix);
      return;
    }
    for (const letter of KEYS[digits[i]]) backtrack(i + 1, prefix + letter);
  }

  backtrack(0, "");
  return result;
}
