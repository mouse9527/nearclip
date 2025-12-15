import SwiftUI

@main
struct NearClipApp: App {
    @NSApplicationDelegateAdaptor(AppDelegate.self) var appDelegate

    var body: some Scene {
        // Empty scene since we use menubar only
        Settings {
            EmptyView()
        }
    }
}
