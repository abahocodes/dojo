# Hints

## Hint 1
Process the text with a single index that only moves forward, handling one
rule at a time: spaces, then an optional sign, then digits.

## Hint 2
Build the number digit by digit with `value = value * 10 + digit`. A second
sign character, a space after the sign, or a `"."` all count as "not a digit"
and end the reading.

## Hint 3
The digits can describe a number far bigger than 32 bits. As soon as the
running value exceeds `2^31 - 1`, you already know the clamped result — return
it right away instead of continuing (in languages with fixed-size integers,
this is also what prevents overflow).
