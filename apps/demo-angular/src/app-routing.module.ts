import { NgModule } from '@angular/core';
import { Routes } from '@angular/router';
import { NativeScriptRouterModule } from '@nativescript/angular';

import { HomeComponent } from './home.component';

const routes: Routes = [
  { path: '', redirectTo: '/home', pathMatch: 'full' },
  { path: 'home', component: HomeComponent },
  { path: 'node-buffer', loadChildren: () => import('./plugin-demos/node-buffer.module').then((m) => m.NodeBufferModule) },
  { path: 'node-core', loadChildren: () => import('./plugin-demos/node-core.module').then((m) => m.NodeCoreModule) },
  { path: 'node-fs', loadChildren: () => import('./plugin-demos/node-fs.module').then((m) => m.NodeFsModule) },
];

@NgModule({
  imports: [NativeScriptRouterModule.forRoot(routes)],
  exports: [NativeScriptRouterModule],
})
export class AppRoutingModule {}
