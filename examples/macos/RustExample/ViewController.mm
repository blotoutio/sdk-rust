//
//  ViewController.m
//  RustExample
//
//  Created by ankuradhikari on 22/10/20.
//

#import "ViewController.h"
//#import "BlotoutAnalytics.h"
#include "wrapper.hpp"


@implementation ViewController

- (void)viewDidLoad {
    [super viewDidLoad];

    // Do any additional setup after loading the view.
}
- (IBAction)initSDK:(id)sender {
    
    blotout::BlotoutAnalytics object = blotout::BlotoutAnalytics();
    object.logEnabled(true);
    
    
    dispatch_async(dispatch_get_main_queue(), ^{
        const char* token = "BEZAVGGW4GZZZ3N";
        const char* endPoint = "http://stage.blotout.io";
        const char* bundleID = "com.blotout.rustsaleDemoApp";
        blotout::BlotoutAnalytics object = blotout::BlotoutAnalytics();
        object.initSDK(token, endPoint, bundleID);
        //bo_sdk_init(token, endPoint, bundleID);
        
    });
}

- (IBAction)publishDeveloperEvents:(id)sender {
    
    @try {
        dispatch_async(dispatch_get_main_queue(), ^{
            const char* eventName = "Rust Event from sdk testing";
            const char* jsonString = "{\"some property\": \"some value\", \"some other property\": \"some other value\"}";
            blotout::BlotoutAnalytics object = blotout::BlotoutAnalytics();
            object.logEvent(eventName, jsonString);
        });
    }@catch(NSException* exp) {
            
    }
    
    
}
- (IBAction)sendPIIData:(id)sender {
    
    @try {
        dispatch_async(dispatch_get_main_queue(), ^{
            const char* eventName = "Rust Event from sdk testing";
            const char* jsonString = "{\"email id\": \"ankuradhikari08@gmail.com\", \"gender\": \"male\"}";
            blotout::BlotoutAnalytics object = blotout::BlotoutAnalytics();
            object.logPiiEvent(eventName, jsonString);
        });
    }@catch(NSException* exp) {
            
    }
    
}
- (IBAction)sendPHIData:(id)sender {
    @try {
        dispatch_async(dispatch_get_main_queue(), ^{
                const char* eventName = "Rust Event from sdk testing";
                const char* jsonString = "{\"email id\": \"ankur@blotout.io\", \"gender\": \"male\"}";
                blotout::BlotoutAnalytics object = blotout::BlotoutAnalytics();
                object.logPhiEvent(eventName, jsonString);
        });
    }@catch(NSException* exp) {
            
    }
}

- (void)setRepresentedObject:(id)representedObject {
    [super setRepresentedObject:representedObject];

    // Update the view, if already loaded.
}


@end
