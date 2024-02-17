import { Component } from "@angular/core";
import { CommonModule } from "@angular/common";
import { RouterOutlet } from "@angular/router";
import { invoke } from "@tauri-apps/api/tauri";

@Component({
  selector: "app-root",
  standalone: true,
  imports: [CommonModule, RouterOutlet],
  templateUrl: "./app.component.html",
  styleUrl: "./app.component.css",
})
export class AppComponent {
  isRunning: boolean = false;

  discordTokenInput: string = "";
  accountSearchInput: string = "";

  errorMessage: string = "";

  searchAccount(event: Event, accountInput: string): void {
    event.preventDefault();
    // todo: account search???
  }

  startServer(event: SubmitEvent, token: string): void {
    event.preventDefault();
    this.isRunning = true;

    invoke<boolean>("start_server", { token })
      .then((isStarted) => {
        if (isStarted) {
          this.errorMessage = "";
          // todo: report success
        } else {
          // todo: report failure
          this.errorMessage =
            "Unexplained error when attempting to start the server.";
        }
      })
      .catch((err) => {
        this.errorMessage = err;
      });
  }

  stopServer(event: Event): void {
    event.preventDefault;
    this.isRunning = false;

    invoke<boolean>("stop_server")
      .then((isStopped) => {
        this.errorMessage = "";
        if (isStopped) {
          // todo: report success
        } else {
          this.errorMessage =
            "Unexplained error when attempting to stop the server.";
          // todo: report error
        }
      })
      .catch((err) => {
        this.errorMessage = err;
      });
  }
}
