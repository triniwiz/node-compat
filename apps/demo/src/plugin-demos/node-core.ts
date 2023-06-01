import { Observable, EventData, Page } from '@nativescript/core';
import { DemoSharedNodeCore } from '@demo/shared';
import {} from '@nativescript/node-core';

export function navigatingTo(args: EventData) {
  const page = <Page>args.object;
  page.bindingContext = new DemoModel();
}

export class DemoModel extends DemoSharedNodeCore {}
