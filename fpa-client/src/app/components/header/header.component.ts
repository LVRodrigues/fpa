import { ChangeDetectionStrategy, Component, inject } from '@angular/core';
import { MatToolbarModule } from '@angular/material/toolbar';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatDividerModule } from '@angular/material/divider';
import { AppComponent } from '../../app.component';
import { AuthService } from '../../services/auth.service';

@Component({
	selector: 'app-header',
	imports: [MatToolbarModule, MatButtonModule, MatIconModule, MatDividerModule],
	templateUrl: './header.component.html',
	styleUrl: './header.component.scss',
	changeDetection: ChangeDetectionStrategy.OnPush,
})
export class HeaderComponent {

	title: String | undefined = "";

	constructor(
		private app: AppComponent,
		private auth: AuthService
	) {
		this.title = app.title;
	}

	signin(): void {
		this.auth.login();
	}

	signout(): void {
		this.auth.logout();
	}

	isAuthenticated(): boolean {
		return this.auth.isAuthenticated();
	}

	username(): string | undefined {
		return this.auth.username();
	}
}
