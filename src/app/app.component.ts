import { Component } from "@angular/core";
import { CommonModule } from "@angular/common";
import { RouterOutlet } from "@angular/router";
import { invoke } from "@tauri-apps/api/tauri";
import { HttpClient, HttpClientModule } from "@angular/common/http";
import { Observable } from "rxjs";

@Component({
  selector: "app-root",
  standalone: true,
  imports: [CommonModule, RouterOutlet, HttpClientModule],
  templateUrl: "./app.component.html",
  styleUrl: "./app.component.css",
})
export class AppComponent {
  isLoading: boolean = false;
  isRunning: boolean = false;
  accountNotFound: boolean = false;

  account: IGAccount | null = null;
  image: any;

  errorMessage: string = "";

  constructor(private http: HttpClient) {}

  searchAccount(event: Event, accountInput: string): void {
    event.preventDefault();

    this.isLoading = true;
    invoke<IGAccount>("search_account", { account: accountInput })
      .then((acct) => {
        this.isLoading = false;
        this.accountNotFound = true;
        this.errorMessage = "";
        this.account = acct;
        this.getImage().subscribe((data: any) => {
          this.createImage(data);
        });
      })
      .catch((err) => {
        this.isLoading = false;
        this.errorMessage = err;
        this.account = null;
        this.accountNotFound = true;
      });
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

  private createImage(image: Blob) {
    let reader = new FileReader();
    reader.addEventListener(
      "load",
      () => {
        this.image = reader.result;
      },
      false,
    );

    if (image) {
      reader.readAsDataURL(image);
    }
  }

  private getImage(): Observable<Blob> {
    if (this.account === null) {
      //this won't happen...
      this.account = {
        username: "",
        profile_pic: "",
        bio: "",
      };
    }
    return this.http.get(this.account.profile_pic, {
      responseType: "blob",
    });
  }
}

interface IGAccount {
  username: string;
  profile_pic: string;
  bio: string;
}
