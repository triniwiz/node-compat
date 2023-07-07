declare const __non_webpack_require__;
import { NodeCoreCommon } from './common';

// load with system before requiring which calls dlopen
java.lang.System.loadLibrary('nodenative');

__non_webpack_require__('system_lib://libnodenativev8.so');

export class NodeCore extends NodeCoreCommon {}
