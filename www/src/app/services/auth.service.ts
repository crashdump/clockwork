import {Injectable, signal, WritableSignal} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {environment} from "../../environments/environment";
import {AuthToken, LoginStatus} from "../models/credentials";


@Injectable({
  providedIn: 'root'
})
export class AuthService {
  public baseUrl = `${environment.baseUrl}/login`;

  readonly isAuthenticated: WritableSignal<LoginStatus> = signal<LoginStatus>(LoginStatus.UNKNOWN);

  private _username: string = '';
  private _password: string = '';

  constructor(private http: HttpClient) {
  }

  login(username: string, password: string): void {
    this.http.post<AuthToken>(this.baseUrl, {username, password})
      .subscribe(
        (response) => {
          if (response.token != "") {
            console.log("auth: successfully logged-in.")
            this._username = username;
            this._password = password;
            this.isAuthenticated.set(LoginStatus.VALID);
          }
        },
        (error) => {
          this.isAuthenticated.set(LoginStatus.FAILED);
          console.log("auth: invalid username or password.")
        },
      );
  }

  logout(): void {
    this.isAuthenticated.set(LoginStatus.UNKNOWN);
    this._username = '';
    this._password = '';
  }

  // isAuthenticated(): WritableSignal<LoginStatus> {
  //   return this._authenticated;
  // }

  getBasicAuthUserPass(): string {
    const up = btoa(`${this._username}:${this._password}`)
    return `Basic ${up}`;
  }
}
