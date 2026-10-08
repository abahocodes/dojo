function removeDuplicateLetters(s) {
  const last = new Array(26).fill(-1);
  for (let i = 0; i < s.length; i++) last[s.charCodeAt(i) - 97] = i;
  const used = new Array(26).fill(false);
  const stack = [];
  for (let i = 0; i < s.length; i++) {
    const c = s.charCodeAt(i) - 97;
    if (used[c]) continue;
    while (stack.length && stack[stack.length - 1] > c && last[stack[stack.length - 1]] > i) {
      used[stack.pop()] = false;
    }
    stack.push(c);
    used[c] = true;
  }
  return String.fromCharCode(...stack.map((c) => c + 97));
}
