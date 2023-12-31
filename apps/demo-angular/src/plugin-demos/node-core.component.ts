import { Component, NgZone } from '@angular/core';
import { DemoSharedNodeCore } from '@demo/shared';
import {} from '@nativescript/node-core';

@Component({
  selector: 'demo-node-core',
  templateUrl: 'node-core.component.html',
})
export class NodeCoreComponent {
  demoShared: DemoSharedNodeCore;

  constructor(private _ngZone: NgZone) {}

  ngOnInit() {
    this.demoShared = new DemoSharedNodeCore();
  }
}
