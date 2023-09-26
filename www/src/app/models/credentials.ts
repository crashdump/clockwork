export class Credentials {
  username!: string;
  password!: string;
}

export class AuthToken {
  token!: string
}

export const enum LoginStatus {
  UNKNOWN = "UNKNOWN",
  FAILED = "FAILED",
  VALID = "VALID",
}
