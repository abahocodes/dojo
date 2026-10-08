You are given `accounts`, where each entry is a list of strings: a person's
name followed by one or more email addresses, `[name, email1, email2, ...]`.

Two accounts belong to the same person when they share at least one email
address, and this is transitive: if account A shares an email with B, and B
shares one with C, all three are the same person. Accounts with the same name
but no chain of shared emails are different people. Accounts that share an
email always have the same name.

Merge each person's accounts into one: `[name, emails...]`, where the emails
are that person's distinct addresses in **sorted order**. Return the list of
merged accounts, **sorted by name, then by first email**.

All sorting compares strings character by character by character code
(plain ASCII order, so uppercase letters come before lowercase ones).

## Example 1

```
accounts = [
  ["Nia", "nia@x.io", "nia.w@y.io"],
  ["Omar", "omar@z.io"],
  ["Nia", "nw@q.io", "nia@x.io"],
  ["Nia", "nia2@x.io"]
]
output = [
  ["Nia", "nia.w@y.io", "nia@x.io", "nw@q.io"],
  ["Nia", "nia2@x.io"],
  ["Omar", "omar@z.io"]
]
# the first and third accounts share nia@x.io; the fourth Nia shares nothing
```

## Example 2

```
accounts = [
  ["Lee", "c@k.com", "a@k.com"],
  ["Kim", "kim@k.com"],
  ["Lee", "b@k.com", "c@k.com"],
  ["Lee", "b@k.com", "d@k.com"]
]
output = [
  ["Kim", "kim@k.com"],
  ["Lee", "a@k.com", "b@k.com", "c@k.com", "d@k.com"]
]
# account 1 and 3 share c@k.com, 3 and 4 share b@k.com
```

## Constraints

- `1 <= len(accounts) <= 1000`
- `2 <= len(accounts[i]) <= 10`
- Names and emails are non-empty ASCII strings of at most 30 characters.
- An email may appear more than once, even within one account.
