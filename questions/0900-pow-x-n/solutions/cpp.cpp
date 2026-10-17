class Solution {
public:
    double myPow(double x, int n) {
        // Binary exponentiation: square the base and halve the exponent,
        // multiplying the base into the result whenever the low bit is set.
        // Widen to long long first: -(-2^31) does not fit in an int.
        long long e = n;
        if (e < 0) {
            x = 1 / x;
            e = -e;
        }
        double result = 1.0;
        while (e > 0) {
            if (e & 1) result *= x;
            x *= x;
            e >>= 1;
        }
        return result;
    }
};
