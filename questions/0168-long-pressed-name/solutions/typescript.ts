function isLongPressedName(name: string, typed: string): boolean {
  let i = 0;
  for (let j = 0; j < typed.length; j++) {
    const c = typed[j];
    if (i < name.length && name[i] === c) i++;
    else if (j === 0 || typed[j - 1] !== c) return false;
  }
  return i === name.length;
}
