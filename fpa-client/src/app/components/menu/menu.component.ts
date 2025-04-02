import { ChangeDetectionStrategy, Component, effect, OnInit, signal } from '@angular/core';
import { MatToolbarModule } from '@angular/material/toolbar';
import { MatIconModule } from '@angular/material/icon';
import { MatButtonModule } from '@angular/material/button';
import { NavigationEnd, Router } from '@angular/router';
import { BreadcrumbService, Step } from '../../services/breadcrumb.service';

@Component({
    selector: 'app-menu',
    imports: [MatToolbarModule, MatIconModule, MatButtonModule],
    templateUrl: './menu.component.html',
    styleUrl: './menu.component.scss',
    changeDetection: ChangeDetectionStrategy.OnPush,
})
export class MenuComponent {

    showProject = signal(false);
    showBoundary = signal(false);
    showFunction = signal(false);

    constructor(
        private router: Router,
        private breadcrumb: BreadcrumbService,
    ) {
        effect(() => {
            console.log('Current step: ', this.breadcrumb.current());
            const current = this.breadcrumb.current()();
            this.showProject.set(current > Step.NONE);
            this.showBoundary.set(current > Step.PROJECT);
            this.showFunction.set(current > Step.BOUNDARY);
        })
    }

    navigateToHome() {
        this.router.navigate(['/home']);
    }

    navigateToProject() {
        this.router.navigate(['/projects']);
    }

    navigateToBoundary() {
        this.router.navigate(['/boundaries']);
    }

    navigateToFunction() {
        this.router.navigate(['/functions']);
    }
}
