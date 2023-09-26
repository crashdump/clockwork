import {Injectable} from '@angular/core';
import {HttpEvent, HttpHandler, HttpInterceptor, HttpRequest} from '@angular/common/http';
import {Observable} from 'rxjs';
import {AuthService} from "../services/auth.service";
import {LoginStatus} from "../models/credentials";

@Injectable()
export class AuthInterceptor implements HttpInterceptor {

  constructor(
    private authService: AuthService,
  ) {}

  intercept(
    req: HttpRequest<unknown>,
    next: HttpHandler,
    ): Observable<HttpEvent<unknown>> {
    if (this.authService.isAuthenticated() == LoginStatus.VALID) {
      console.log("User is authenticated, injecting Authorization header.")

      let nreq = req.clone({ setHeaders: {
        Authorization: this.authService.getBasicAuthUserPass()
      } });

      return next.handle(nreq);
    }
    return next.handle(req);
  }
}
