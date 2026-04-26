function collatz_steps(n: number): number {
    if (n === 1) {
        return 0;
    }

    // The Collatz conjecture is typically defined for positive integers.
    // For robustness, one might add checks for n <= 0 or non-integer n,
    // but for the given problem context and test cases, we assume valid positive integer input.

    let steps = 0;
    let current_n = n;

    while (current_n !== 1) {
        if (current_n % 2 === 0) {
            current_n /= 2;
        } else {
            current_n = 3 * current_n + 1;
        }
        steps++;
    }

    return steps;
}

console.log(collatz_steps(1));
console.log(collatz_steps(2));
console.log(collatz_steps(6));
console.log(collatz_steps(10));