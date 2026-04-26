function is_palindrome(s: string): boolean {
  const reversed = s.split("").reverse().join("");
  return s === reversed;
}

console.log(is_palindrome("racecar") ? "true" : "false");
console.log(is_palindrome("hello") ? "true" : "false");
console.log(is_palindrome("") ? "true" : "false");
console.log(is_palindrome("a") ? "true" : "false");
