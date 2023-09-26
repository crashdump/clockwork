import {HttpClient, HttpHeaders} from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import {Task} from "../models/task";
import {environment} from "../../environments/environment";
import {AuthService} from "./auth.service";

@Injectable({
  providedIn: 'root'
})
export class TaskService {
  public baseUrl = `${environment.baseUrl}/tasks`;

  constructor(
    private httpClient: HttpClient,
    ) {
  }

  public list(): Observable<string[]> {
    return this.httpClient.get<string[]>(this.baseUrl);
  }

  public get(id: string): Observable<Task> {
    return this.httpClient.get<Task>(`${this.baseUrl}/${id}`);
  }

  public rearm(id: string): Observable<Task> {
    return this.httpClient.get<Task>(`${this.baseUrl}/${id}/reset`);
  }
}
