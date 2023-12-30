declare const NodeNativeModule;
import { NodeCoreCommon } from './common';

const cm = new NodeNativeModule();
cm.install();

export class NodeCore extends NodeCoreCommon {}
