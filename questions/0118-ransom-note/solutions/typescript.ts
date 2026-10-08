function canConstruct(ransomNote: string, magazine: string): boolean {
  if (ransomNote.length > magazine.length) return false;
  const counts: number[] = new Array(26).fill(0);
  for (let i = 0; i < magazine.length; i++) counts[magazine.charCodeAt(i) - 97]++;
  for (let i = 0; i < ransomNote.length; i++) {
    const k = ransomNote.charCodeAt(i) - 97;
    if (--counts[k] < 0) return false;
  }
  return true;
}
