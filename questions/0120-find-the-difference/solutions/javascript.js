function findTheDifference(s, t) {
  let x = 0;
  for (let i = 0; i < s.length; i++) x ^= s.charCodeAt(i);
  for (let i = 0; i < t.length; i++) x ^= t.charCodeAt(i);
  return String.fromCharCode(x);
}
