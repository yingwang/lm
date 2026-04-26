type Result<T> = { type: "Ok"; value: T } | { type: "Err"; message: string };

function validateUsername(username: string): Result<string> {
  if (username === "") {
    return { type: "Err", message: "username must not be empty" };
  }
  if (username.length > 20) {
    return { type: "Err", message: "username must be at most 20 characters" };
  }
  return { type: "Ok", value: username };
}

function validateEmail(email: string): Result<string> {
  if (!email.includes("@")) {
    return { type: "Err", message: "email must contain @" };
  }
  return { type: "Ok", value: email };
}

function validatePassword(password: string): Result<string> {
  if (password.length < 8) {
    return { type: "Err", message: "password must be at least 8 characters" };
  }
  return { type: "Ok", value: password };
}

function register(username: string, email: string, password: string): Result<string> {
  const usernameResult = validateUsername(username);
  if (usernameResult.type === "Err") {
    return usernameResult;
  }

  const emailResult = validateEmail(email);
  if (emailResult.type === "Err") {
    return emailResult;
  }

  const passwordResult = validatePassword(password);
  if (passwordResult.type === "Err") {
    return passwordResult;
  }

  return {
    type: "Ok",
    value: `${username} registered with ${email}`,
  };
}

function resultToString(result: Result<string>): string {
  if (result.type === "Ok") {
    return `Ok(${result.value})`;
  }
  return `Err(${result.message})`;
}

// Test cases
const testCases = [
  ["alice", "alice@example.com", "password123"],
  ["", "a@b", "pass1234"],
  ["alice", "noatsign", "pass1234"],
  ["alice", "a@b", "short"],
  ["aaaaabbbbbcccccddddde", "a@b", "pass1234"],
];

for (const [username, email, password] of testCases) {
  console.log(resultToString(register(username, email, password)));
}
