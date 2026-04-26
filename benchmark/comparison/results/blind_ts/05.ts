function caesar_encrypt(text: string, shift: number): string {
  let result = "";
  for (const c of text) {
    const code = c.charCodeAt(0);
    if (code >= 65 && code <= 90) {
      result += String.fromCharCode(((code - 65 + shift) % 26 + 26) % 26 + 65);
    } else {
      result += c;
    }
  }
  return result;
}

console.log(caesar_encrypt("ABC", 1));
console.log(caesar_encrypt("XYZ", 3));
console.log(caesar_encrypt("HELLO", 13));
console.log(caesar_encrypt("A B!", 2));
