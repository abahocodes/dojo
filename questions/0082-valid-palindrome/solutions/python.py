def is_palindrome(s: str) -> bool:
    def keep(ch: str) -> bool:
        return ch.isascii() and ch.isalnum()

    left, right = 0, len(s) - 1
    while left < right:
        if not keep(s[left]):
            left += 1
        elif not keep(s[right]):
            right -= 1
        elif s[left].lower() != s[right].lower():
            return False
        else:
            left += 1
            right -= 1
    return True
