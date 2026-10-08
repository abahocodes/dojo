function removeSubfolders(folder) {
  const sorted = [...folder].sort();
  const result = [];
  for (const path of sorted) {
    if (result.length === 0 || !path.startsWith(result[result.length - 1] + "/")) {
      result.push(path);
    }
  }
  return result;
}
