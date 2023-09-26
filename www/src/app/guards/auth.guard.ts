import {CanActivateFn, Router, UrlTree} from '@angular/router';
import {inject} from "@angular/core";
import {AuthService} from "../services/auth.service";
import {LoginStatus} from "../models/credentials";

export const authGuard: CanActivateFn = (
  route,
  state,
  ) => {
  const authService: AuthService = inject(AuthService);
  const router: Router = inject(Router);
  return authService.isAuthenticated() == LoginStatus.VALID || router.createUrlTree(['/','login']);
};
