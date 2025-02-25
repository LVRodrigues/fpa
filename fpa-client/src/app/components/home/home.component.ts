import { afterNextRender, ChangeDetectionStrategy, Component } from '@angular/core';
import mermaid from 'mermaid';

@Component({
    selector: 'app-home',
    imports: [],
    templateUrl: './home.component.html',
    styleUrl: './home.component.scss',
    changeDetection: ChangeDetectionStrategy.OnPush,
})
export class HomeComponent {

    constructor() {
        afterNextRender({
            read: () => {
                void mermaid.initialize({
                    startOnLoad: true,
                });
            },
        })
    }
}
