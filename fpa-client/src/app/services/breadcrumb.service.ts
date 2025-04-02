import { Injectable, Signal, signal, WritableSignal } from '@angular/core';
import { ActivatedRoute, NavigationEnd, Router } from '@angular/router';
import { filter, map } from 'rxjs';

export enum Step {
	NONE,
	PROJECT,
	BOUNDARY,
	FUNCTION
}

@Injectable({
	providedIn: 'root'
})
export class BreadcrumbService {

	private step: WritableSignal<Step> = signal(Step.NONE);

	constructor(
		private router: Router
	) {
		this.router.events
			.pipe(
				filter(event => event instanceof NavigationEnd),
				map(() => {
					let step = Step.NONE;
					let route: ActivatedRoute = this.router.routerState.root;
					while (route!.firstChild) {
						route = route.firstChild;
					}
					if (route.snapshot.data['step']) {
						step = route.snapshot.data['step'];
					}
					return step;
				}))
			.subscribe((step: Step) => {
				console.debug('BreadcrumbService', 'step', step);
				this.step.set(step);
			});
	}

	current(): Signal<Step>{
		return this.step.asReadonly();
	}
}
