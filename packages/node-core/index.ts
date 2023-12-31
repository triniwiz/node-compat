declare const __non_webpack_require__, NodeNativeModule;

if (global.isAndroid) {
  // load with system before requiring which calls dlopen
  java.lang.System.loadLibrary('nodenative');

  __non_webpack_require__('system_lib://libnodenativev8.so');
}

if (global.isIOS) {
  const cm = new NodeNativeModule();
  cm.install();
}
