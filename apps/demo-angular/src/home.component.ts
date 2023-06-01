import { Component } from '@angular/core';

@Component({
  selector: 'demo-home',
  templateUrl: 'home.component.html',
})
export class HomeComponent {
  demos = [
    {
      name: 'node-buffer',
    },
    {
      name: 'node-core',
    },
    {
      name: 'node-fs',
    },
  ];
}
