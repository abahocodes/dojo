KEYS = {
    "2": "abc", "3": "def", "4": "ghi", "5": "jkl",
    "6": "mno", "7": "pqrs", "8": "tuv", "9": "wxyz",
}


def letter_combinations(digits: str) -> list[str]:
    if not digits:
        return []
    result, current = [], []

    def backtrack(i):
        if i == len(digits):
            result.append("".join(current))
            return
        for letter in KEYS[digits[i]]:
            current.append(letter)
            backtrack(i + 1)
            current.pop()

    backtrack(0)
    return result
