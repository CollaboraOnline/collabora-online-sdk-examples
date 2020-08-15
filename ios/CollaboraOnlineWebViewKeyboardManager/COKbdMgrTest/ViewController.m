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

#import "ViewController.h"

@interface ViewController () <WKNavigationDelegate, WKUIDelegate, UIScrollViewDelegate> {

    WKWebView *webView;
    CollaboraOnlineWebViewKeyboardManager *keyboardManager;
}

@end

@implementation ViewController

- (void)viewDidAppear:(BOOL)animated {

    WKWebViewConfiguration *configuration = [[WKWebViewConfiguration alloc] init];
    WKUserContentController *userContentController = [[WKUserContentController alloc] init];

    configuration.userContentController = userContentController;

    webView = [[WKWebView alloc] initWithFrame:CGRectZero configuration:configuration];
    webView.translatesAutoresizingMaskIntoConstraints = NO;
    webView.allowsLinkPreview = NO;

    webView.scrollView.delegate = self;

    [self.view addSubview:self->webView];

    webView.navigationDelegate = self;
    webView.UIDelegate = self;

    WKWebView *webViewP = webView;
    NSDictionary *views = NSDictionaryOfVariableBindings(webViewP);
    [self.view
        addConstraints:[NSLayoutConstraint constraintsWithVisualFormat:@"H:|-0-[webViewP(>=0)]-0-|"
                                                               options:0
                                                               metrics:nil
                                                                 views:views]];
    [self.view
        addConstraints:[NSLayoutConstraint constraintsWithVisualFormat:@"V:|-0-[webViewP(>=0)]-0-|"
                                                               options:0
                                                               metrics:nil
                                                                 views:views]];

    NSURL *url = [[NSBundle mainBundle] URLForResource:@"test" withExtension:@"html"];
    NSURLComponents *components =
        [NSURLComponents componentsWithURL:url resolvingAgainstBaseURL:NO];

    NSURLRequest *request = [[NSURLRequest alloc] initWithURL:components.URL];
    [webView loadRequest:request];

    // The function names passed to CollaboraOnlineWebViewKeyboardManager's initForWebView method
    // are those we know are in the test.html.

    keyboardManager =
        [[CollaboraOnlineWebViewKeyboardManager alloc] initForWebView:webView
                                                     executeAfterInit:nil
                                                 executeForInsertText:@"keyboardInsertText"
                                             executeForDeleteBackward:@"keyboardDeleteBackward"];
}

@end

// vim:set shiftwidth=4 softtabstop=4 expandtab:
