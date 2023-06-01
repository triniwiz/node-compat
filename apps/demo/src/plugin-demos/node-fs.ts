import { Observable, EventData, Page } from '@nativescript/core';
import { DemoSharedNodeFs } from '@demo/shared';
import {} from '@nativescript/node-fs';

export function navigatingTo(args: EventData) {
  const page = <Page>args.object;
  page.bindingContext = new DemoModel();
}

export class DemoModel extends DemoSharedNodeFs {}
