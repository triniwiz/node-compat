import { Observable, EventData, Page } from '@nativescript/core';
import { DemoSharedNodeBuffer } from '@demo/shared';
import {} from '@nativescript/node-buffer';

export function navigatingTo(args: EventData) {
  const page = <Page>args.object;
  page.bindingContext = new DemoModel();
}

export class DemoModel extends DemoSharedNodeBuffer {}
