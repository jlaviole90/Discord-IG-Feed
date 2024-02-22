import { Component } from "@angular/core";
import { CommonModule } from "@angular/common";
import { invoke } from "@tauri-apps/api/tauri";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { FontAwesomeModule } from "@fortawesome/angular-fontawesome";
import { faTrash } from "@fortawesome/free-solid-svg-icons";

@Component({
  selector: "app-root",
  standalone: true,
  imports: [CommonModule, FontAwesomeModule],
  templateUrl: "./app.component.html",
  styleUrl: "./app.component.css",
})
export class AppComponent {
  isLoading: boolean = false;
  isRunning: boolean = false;
  accountNotFound: boolean = false;

  accountInput: string = "";
  prefixInput: string = "";
  tokenInput: string = "";

  account: IGAccount = BlankData;
  imagePath: string = "";

  serverLogs: string = "";
  errorMessage: string = "";

  faTrash = faTrash;

  searchAccount(event: Event): void {
    event.preventDefault();

    if (this.accountInput === "") {
      this.errorMessage = "No account provided...";
      return;
    }

    this.isLoading = true;
    invoke<IGAccount>("search_account", { account: this.accountInput })
      .then((acct) => {
        this.isLoading = false;
        this.accountNotFound = true;
        this.errorMessage = "";
        this.account = acct;
        this.imagePath = convertFileSrc(this.account.profile_pic);
      })
      .catch((err) => {
        this.isLoading = false;
        this.errorMessage = err;
        this.accountNotFound = true;
      });
  }

  removeAccount(): void {
    this.account = BlankData;
  }

  startServer(event: Event): void {
    event.preventDefault();
    this.isRunning = true;

    if (
      this.tokenInput === "" ||
      this.account.username === "" ||
      this.prefixInput === ""
    ) {
      this.errorMessage = "Missing data!";
      return;
    }

    invoke<boolean>("start_server", {
      token: this.tokenInput,
      account: this.account.username,
      prefix: this.prefixInput,
    })
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

  getValue(event: Event): string {
    return (event.target as HTMLInputElement).value;
  }
}

export interface IGAccount {
  username: string;
  bio: string;
  profile_pic: string;
}
export const BlankData: IGAccount = {
  username: "",
  bio: "",
  profile_pic: "",
};
