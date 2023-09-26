import {Component} from '@angular/core';
import {Router} from '@angular/router';
import {Task} from "../../models/task";
import {TaskService} from "../../services/task.service";
import {CountdownConfig} from "ngx-countdown";

@Component({
  selector: 'app-task-list',
  templateUrl: './task-list.component.html',
  styleUrls: ['./task-list.component.css']
})
export class TaskListComponent {

  ids: string[] = [];
  tasks: Task[] = [];

  constructor(
    private taskService: TaskService,
    private router: Router,
  ) {
  }

  ngOnInit() {
    this.taskRefresh()
  }

  countdownConfigFactory(remaining: number): CountdownConfig {
    return {
      format: `hh:mm:ss.s`,
      leftTime: remaining,
    };
  }

  taskRefresh() {
    this.taskService.list()
      .subscribe(ids => {
        this.ids = ids;
        ids.forEach((id) => {
          this.taskService.get(id).pipe().subscribe(
            (data) => {
              let task = data
              task.expires_at = new Date(Date.now() + task.expires_in);
              this.tasks.push(task)
            }
          )
        });
      });
  }

  taskDetails(id: string) {
    this.router.navigate(['tasks', id]);
    console.log(id);
  }

  protected readonly Date = Date;
}
