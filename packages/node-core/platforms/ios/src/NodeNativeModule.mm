#import "NodeNativeModule.h"
#import <NativeScript/runtime/Runtime.h>
#import "NodeNativeJSIModule.h"

using namespace std;

@implementation NodeNativeModule

- (void )install {
    v8::Isolate* isolate = tns::Runtime::GetCurrentRuntime()->GetIsolate();
    NodeNativeJSIModule::install(isolate);
}

@end