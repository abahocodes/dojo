A chemical `formula` is written with these rules:

- An **element** is an uppercase letter followed by zero or more lowercase
  letters, e.g. `H`, `Mg`, `Uuo`.
- An element may be followed by a **count** of at least `2` (written without
  leading zeros). With no count, it means one atom.
- Formulas can be written next to each other, e.g. `H2O` or `NaCl`.
- A non-empty formula can be wrapped in parentheses and followed by an
  optional **multiplier** of at least `2`, e.g. `(OH)2`, which multiplies every
  count inside. Parentheses can be nested.

Count the atoms of every element in the formula and return them as a single
string: elements in **alphabetical order** (comparing names character by
character, so `H` comes before `He`), each name followed by its total count
only when that count is greater than `1`.

## Example 1

```
formula = "Mg(OH)2"
output  = "H2MgO2"
```

## Example 2

```
formula = "K4(ON(SO3)2)2"
output  = "K4N2O14S4"   # O: (1 + 3*2) * 2 = 14, S: 2 * 2 = 4, N: 2
```

## Constraints

- `1 <= len(formula) <= 1000`
- `formula` is valid and consists of English letters, digits, `'('` and `')'`
- every element's total count fits in a 32-bit signed integer
