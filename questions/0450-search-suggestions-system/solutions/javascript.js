function suggestedProducts(products, searchWord) {
  // Default sort compares UTF-16 code units: plain lexicographic order here.
  const ordered = [...products].sort();
  const result = [];
  let start = 0;
  for (let k = 1; k <= searchWord.length; k++) {
    const prefix = searchWord.slice(0, k);
    // First index whose word is >= prefix; longer prefixes never move it back.
    let lo = start;
    let hi = ordered.length;
    while (lo < hi) {
      const mid = (lo + hi) >> 1;
      if (ordered[mid] < prefix) lo = mid + 1;
      else hi = mid;
    }
    start = lo;
    const suggestions = [];
    for (let i = start; i < ordered.length && i < start + 3; i++) {
      if (!ordered[i].startsWith(prefix)) break;
      suggestions.push(ordered[i]);
    }
    result.push(suggestions);
  }
  return result;
}
