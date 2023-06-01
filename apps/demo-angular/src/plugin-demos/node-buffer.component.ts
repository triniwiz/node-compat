import { Component, NgZone } from '@angular/core';
import { DemoSharedNodeBuffer } from '@demo/shared';
import {} from '@nativescript/node-buffer';

@Component({
  selector: 'demo-node-buffer',
  templateUrl: 'node-buffer.component.html',
})
export class NodeBufferComponent {
  demoShared: DemoSharedNodeBuffer;

  constructor(private _ngZone: NgZone) {}

  ngOnInit() {
    this.demoShared = new DemoSharedNodeBuffer();
  }
}
