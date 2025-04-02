import { Routes } from '@angular/router';
import { HomeComponent } from './components/home/home.component';
import { ForbiddenComponent } from './components/forbidden/forbidden.component';
import { isAuthenticated } from './guards/auth.guard';
import { ProjectsComponent } from './components/projects/projects.component';
import { Step } from './services/breadcrumb.service';
import { BoundariesComponent } from './components/boundaries/boundaries.component';
import { FunctionsComponent } from './components/functions/functions.component';

export const routes: Routes = [
    { path: '', redirectTo: 'home', pathMatch: 'full' },
    { path: 'home', component: HomeComponent, data: { step: Step.NONE, title: 'Start' } },
    { path: 'projects', component: ProjectsComponent, data: { step: Step.PROJECT, title: 'Project' }, canActivate: [isAuthenticated] },
    { path: 'boundaries', component: BoundariesComponent, data: { step: Step.BOUNDARY, title: 'Boundary' }, canActivate: [isAuthenticated] },
    { path: 'functions', component: FunctionsComponent, data: { step: Step.FUNCTION, title: 'Function' }, canActivate: [isAuthenticated] },
    { path: 'forbidden', component: ForbiddenComponent },
    { path: '**', redirectTo: 'home' }
];
