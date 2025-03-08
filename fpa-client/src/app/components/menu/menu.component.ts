import { ChangeDetectionStrategy, Component, OnInit } from '@angular/core';
import { MatToolbarModule } from '@angular/material/toolbar';
import { MatIconModule } from '@angular/material/icon';
import { MatButtonModule } from '@angular/material/button';
import { NavigationEnd, Router } from '@angular/router';
import { BreadcrumbService } from '../../services/breadcrumb.service';
import { filter } from 'rxjs';

@Component({
    selector: 'app-menu',
    imports: [MatToolbarModule, MatIconModule, MatButtonModule],
    templateUrl: './menu.component.html',
    styleUrl: './menu.component.scss',
    changeDetection: ChangeDetectionStrategy.OnPush,
})
export class MenuComponent {

    showProject: boolean = false;
    showBoundary: boolean = false;
    showFunction: boolean = false;

    constructor(
        private router: Router,
        private breadcrumb: BreadcrumbService,
    ) { 
        this.router.events.pipe(
            filter(event => event instanceof NavigationEnd))
        .subscribe(() => {
            console.debug('MenuComponent', 'constructor', 'router.events');
            this.showProject = this.breadcrumb.canShowProject();
            this.showBoundary = this.breadcrumb.canShowBoundary();
            this.showFunction = this.breadcrumb.canShowFunction();
        });
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
