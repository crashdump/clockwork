import {Component} from '@angular/core';
import {ActivatedRoute, Router} from '@angular/router';
import {Task} from "../../models/task";
import {TaskService} from "../../services/task.service";
import {MatSnackBar} from "@angular/material/snack-bar";

@Component({
  selector: 'app-task-list',
  templateUrl: './task-details.component.html',
  styleUrls: ['./task-details.component.css']
})
export class TaskDetailsComponent {

  id!: string;
  task!: Task;

  constructor(
    public snackBar: MatSnackBar,
    private taskService: TaskService,
    private route: ActivatedRoute,
    private router: Router,
  ) {
  }

  ngOnInit() {
    this.task = new Task();
    this.id = this.route.snapshot.params['id'];

    this.taskService.get(this.id)
      .subscribe(data => {
        console.log(data);
        this.task = data;
      }, error => console.log(error));
  }

  rearmNotPossible() {
    this.snackBar.open("Rearming is not possible when the timer has already reached 0.", "Ok",{ duration: 1500, });
  }

  rearmTask(id: string) {
    console.log(`rearmed ${id}`);
    this.taskService.rearm(id);
  }

  navigateToTaskList() {
    this.router.navigate(['tasks']);
  }

}
