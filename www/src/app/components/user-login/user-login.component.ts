import {Component} from '@angular/core';
import {FormBuilder, FormGroup, Validators} from "@angular/forms";
import {Router} from "@angular/router";
import {MatSnackBar} from "@angular/material/snack-bar";
import {Observable} from "rxjs";

import {AuthService} from "../../services/auth.service";
import {toObservable} from "@angular/core/rxjs-interop";
import {LoginStatus} from "../../models/credentials";


@Component({
  selector: 'app-user-login',
  templateUrl: './user-login.component.html',
  styleUrls: ['./user-login.component.css']
})
export class UserLoginComponent {
  loginForm: FormGroup = this.formBuilder.group({
    username: ['', Validators.required],
    password: ['', Validators.required]
  })

  isAuthenticated$: Observable<LoginStatus> = toObservable(
    this.authService.isAuthenticated
  );

  constructor(
    public snackBar: MatSnackBar,
    private formBuilder: FormBuilder,
    private authService: AuthService,
    private router: Router,
    ) {
    if (this.authService.isAuthenticated() == LoginStatus.VALID) {
      this.router.navigate(['/', 'tasks']);
    }
  }

  ngOnInit(): void {
  }

  onSubmit(): void {
    if(this.loginForm.invalid) {
      this.snackBar.open("Username and password fields are required.");
      return;
    }

    const fval = this.loginForm.value;
    this.authService.login(fval.username, fval.password);

    this.isAuthenticated$.pipe().subscribe(
      (loginStatus) => {
        if (loginStatus == LoginStatus.VALID) {
          this.router.navigate(['/', 'tasks']);
        } else {
          this.snackBar.open("Invalid username or password.", "ok",{ duration: 1500, });
          this.loginForm.reset();
        }
      }
    )
  }
}
