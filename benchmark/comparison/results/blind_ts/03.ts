function is_palindrome(s: string): boolean {
  const rev = s.split("").reverse().join("");
  return s === rev;
}

console.log(String(is_palindrome("racecar")));
console.log(String(is_palindrome("hello")));
console.log(String(is_palindrome("")));
console.log(String(is_palindrome("a")));
