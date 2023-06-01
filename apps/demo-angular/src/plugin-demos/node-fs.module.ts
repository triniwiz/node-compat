import { NgModule, NO_ERRORS_SCHEMA } from '@angular/core';
import { NativeScriptCommonModule, NativeScriptRouterModule } from '@nativescript/angular';
import { NodeFsComponent } from './node-fs.component';

@NgModule({
  imports: [NativeScriptCommonModule, NativeScriptRouterModule.forChild([{ path: '', component: NodeFsComponent }])],
  declarations: [NodeFsComponent],
  schemas: [NO_ERRORS_SCHEMA],
})
export class NodeFsModule {}
