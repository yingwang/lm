function caesar_encrypt(text: string, shift: number): string {
  let result = "";
  for (const c of text) {
    if (c >= "A" && c <= "Z") {
      const code = ((c.charCodeAt(0) - 65 + shift) % 26 + 26) % 26 + 65;
      result += String.fromCharCode(code);
    } else {
      result += c;
    }
  }
  return result;
}

console.log(caesar_encrypt("ABC", 1));
console.log(caesar_encrypt("ABC", 0));
console.log(caesar_encrypt("HELLO", 13));
console.log(caesar_encrypt("B C!", 1));
