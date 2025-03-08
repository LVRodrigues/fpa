import { ChangeDetectionStrategy, Component } from '@angular/core';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { Router } from '@angular/router';

@Component({
	selector: 'app-projects',
	imports: [MatButtonModule, MatIconModule],
	templateUrl: './projects.component.html',
	styleUrl: './projects.component.scss',
	changeDetection: ChangeDetectionStrategy.OnPush,
})
export class ProjectsComponent {

	constructor(
		private router: Router
	) {}

	navigateToBoundaries() {
		this.router.navigate(['/boundaries']);
	}
}
