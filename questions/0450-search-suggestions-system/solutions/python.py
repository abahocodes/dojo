from bisect import bisect_left


def suggested_products(products: list[str], search_word: str) -> list[list[str]]:
    ordered = sorted(products)
    result = []
    start = 0
    for k in range(1, len(search_word) + 1):
        prefix = search_word[:k]
        # Longer prefixes sort no earlier, so the search can resume from start.
        start = bisect_left(ordered, prefix, start)
        suggestions = []
        for word in ordered[start:start + 3]:
            if not word.startswith(prefix):
                break
            suggestions.append(word)
        result.append(suggestions)
    return result
