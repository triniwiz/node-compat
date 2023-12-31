import { NgModule, NO_ERRORS_SCHEMA } from '@angular/core';
import { NativeScriptCommonModule, NativeScriptRouterModule } from '@nativescript/angular';
import { NodeCoreComponent } from './node-core.component';

@NgModule({
  imports: [NativeScriptCommonModule, NativeScriptRouterModule.forChild([{ path: '', component: NodeCoreComponent }])],
  declarations: [NodeCoreComponent],
  schemas: [NO_ERRORS_SCHEMA],
})
export class NodeCoreModule {}
