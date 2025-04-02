import { ChangeDetectionStrategy, Component } from '@angular/core';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { RouterLink } from '@angular/router';

@Component({
  selector: 'app-boundaries',
  imports: [MatButtonModule, MatIconModule, RouterLink],
  templateUrl: './boundaries.component.html',
  styleUrl: './boundaries.component.scss',
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class BoundariesComponent { }
