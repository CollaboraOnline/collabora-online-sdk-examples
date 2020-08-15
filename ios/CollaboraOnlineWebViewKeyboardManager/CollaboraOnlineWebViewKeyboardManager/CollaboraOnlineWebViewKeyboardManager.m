// -*- Mode: ObjC; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 4; fill-column: 100 -*-
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#import <CollaboraOnlineWebViewKeyboardManager/CollaboraOnlineWebViewKeyboardManager.h>

@interface _COWVKMKeyInputControl : UIControl <UIKeyInput> {
    WKWebView *webView;
    NSString *insertTextFunction;
    NSString *deleteBackwardFunction;
}

- (instancetype)initForWebView:(nonnull WKWebView *)webView
        withInsertTextFunction:(nonnull NSString *)insertTextFunction
     andDeleteBackwardFunction:(nonnull NSString *)deleteBackwardFunction;

@end

@implementation _COWVKMKeyInputControl

- (instancetype)initForWebView:(nonnull WKWebView *)webView
        withInsertTextFunction:(nonnull NSString *)insertTextFunction
     andDeleteBackwardFunction:(nonnull NSString *)deleteBackwardFunction {
    self = [super init];

    self->webView = webView;
    self->insertTextFunction = insertTextFunction;
    self->deleteBackwardFunction = deleteBackwardFunction;

    return self;
}

- (void)insertText:(NSString *)text {
    NSLog(@"insertText:'%@'", text);

    NSMutableString *js = [NSMutableString string];

    [js setString:insertTextFunction];
    [js appendString:@"('"];

    for (unsigned i = 0; i < text.length; i++) {
        const unichar c = [text characterAtIndex:i];
        if (c == '\'' || c == '\\') {
            [js appendString:@"\\"];
            [js appendFormat:@"%c", c];
        } else if (c < ' ' || c >= 0x7F) {
            [js appendFormat:@"\\u%04X", c];
        } else {
            [js appendFormat:@"%c", c];
        }
    }

    [js appendString:@"');"];

    [webView evaluateJavaScript:js
              completionHandler:^(id _Nullable obj, NSError *_Nullable error) {
                if (error)
                    NSLog(@"Error when executing '%@': %@", js, [error localizedDescription]);
              }];
}

- (void)deleteBackward {
    NSLog(@"deleteBackward");

    NSString *js = [deleteBackwardFunction stringByAppendingString:@"();"];
    [webView evaluateJavaScript:js
              completionHandler:^(id _Nullable obj, NSError *_Nullable error) {
                if (error)
                    NSLog(@"Error when executing '%@': %@", js, [error localizedDescription]);
              }];
}

- (BOOL)canBecomeFirstResponder {
    return YES;
}

@synthesize hasText;

@end

@interface CollaboraOnlineWebViewKeyboardManager () <WKScriptMessageHandler> {
    WKWebView *webView;
    NSString *jsForInsertText;
    NSString *jsForDeleteBackward;
    _COWVKMKeyInputControl *control;
}

@end

@implementation CollaboraOnlineWebViewKeyboardManager

- (CollaboraOnlineWebViewKeyboardManager *)initForWebView:(nonnull WKWebView *)webView
                                         executeAfterInit:(nullable NSString *)jsAfterInit
                                     executeForInsertText:(nullable NSString *)jsForInsertText
                                 executeForDeleteBackward:(nullable NSString *)jsForDeleteBackward {
    self->webView = webView;
    self->jsForInsertText = jsForInsertText;
    self->jsForDeleteBackward = jsForDeleteBackward;

    [webView.configuration.userContentController
        addScriptMessageHandler:self
                           name:@"CollaboraOnlineWebViewKeyboardManager"];

    // Define functions window.DisplayKeyboard and window.HideKeyboard that will contact our code
    // below to do it. The JS can check for the existence of these functions, and if there, use them
    // instead of any text area focus() and blir() calls that are unreliable or generally act weird.

    if (jsAfterInit) {
        [self->webView evaluateJavaScript:jsAfterInit
                        completionHandler:^(id _Nullable obj, NSError *_Nullable error) {
                          if (error)
                              NSLog(@"Error when executing '%@': %@", jsAfterInit,
                                    [error localizedDescription]);
                        }];
    }

    [[NSNotificationCenter defaultCenter] addObserver:self
                                             selector:@selector(keyboardDidHide:)
                                                 name:UIKeyboardDidHideNotification
                                               object:nil];

    return self;
}

- (void)userContentController:(nonnull WKUserContentController *)userContentController
      didReceiveScriptMessage:(nonnull WKScriptMessage *)message {
    if (![message.name isEqualToString:@"CollaboraOnlineWebViewKeyboardManager"]) {
        NSLog(@"Received unrecognized script message name: %@ %@", message.name, message.body);
        return;
    }

    if ([message.body isEqualToString:@"display"]) {
        if (control == nil) {
            control = [[_COWVKMKeyInputControl alloc] initForWebView:self->webView
                                              withInsertTextFunction:jsForInsertText
                                           andDeleteBackwardFunction:jsForDeleteBackward];
            [self->webView addSubview:control];
            [control becomeFirstResponder];
        }
    } else if ([message.body isEqualToString:@"hide"]) {
        if (control != nil) {
            [control removeFromSuperview];
            control = nil;
        }
    } else {
        NSLog(@"Received unrecognized message body:%@", message.body);
        return;
    }
}

- (void)keyboardDidHide:(NSNotification *)notification {
    NSLog(@"keyboardDidHide");

    if (control != nil) {
        [control removeFromSuperview];
        control = nil;
    }
}

@end

// vim:set shiftwidth=4 softtabstop=4 expandtab:
