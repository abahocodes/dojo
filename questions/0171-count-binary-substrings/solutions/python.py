def count_binary_substrings(s: str) -> int:
    total = 0
    prev_run, cur_run = 0, 1
    for i in range(1, len(s)):
        if s[i] == s[i - 1]:
            cur_run += 1
        else:
            total += min(prev_run, cur_run)
            prev_run, cur_run = cur_run, 1
    return total + min(prev_run, cur_run)
