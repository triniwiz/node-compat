import { Component, NgZone } from '@angular/core';
import { DemoSharedNodeFs } from '@demo/shared';
import {} from '@nativescript/node-fs';

@Component({
  selector: 'demo-node-fs',
  templateUrl: 'node-fs.component.html',
})
export class NodeFsComponent {
  demoShared: DemoSharedNodeFs;

  constructor(private _ngZone: NgZone) {}

  ngOnInit() {
    this.demoShared = new DemoSharedNodeFs();
  }
}
