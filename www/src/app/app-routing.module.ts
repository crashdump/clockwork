import {NgModule} from '@angular/core';
import {RouterModule, Routes} from '@angular/router';

import {authGuard} from "./guards/auth.guard";
import {TaskListComponent} from "./components/task-list/task-list.component";
import {TaskDetailsComponent} from "./components/task-details/task-details.component";
import {UserLoginComponent} from "./components/user-login/user-login.component";
import {PageNotFoundComponent} from "./components/page-not-found/page-not-found.component";

const routes: Routes = [
  {
    path: '',
    pathMatch: 'full',
    redirectTo: 'tasks',
    canActivate: [authGuard],
  },
  {
    path: 'login',
    component: UserLoginComponent,
  },
  {
    path: 'tasks',
    component: TaskListComponent,
    canActivate: [authGuard],
  },
  {
    path: 'tasks/:id',
    component: TaskDetailsComponent,
    canActivate: [authGuard],
  },
  {
    path: '**',
    component: PageNotFoundComponent,
  }
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule {
}
