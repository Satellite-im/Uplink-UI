# Uplink UI

![Uplink UI](https://i.imgur.com/X4AGeLz.png)

Implementation of a UI atop Warp using a standardized State model and UIKit to reinforce reusable component usage.

---

## Quickstart

To get running fast ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.


**Standard Run:**
```
cargo run --bin ui
```

**Rapid Release Testing:**
This version will run closely to release but without recompiling crates every time.
```
cargo run --bin ui --profile=rapid
```

---


## Dependancy List

**MacOS M1+**
| Dep  | Install Command                                                  |
|------|------------------------------------------------------------------|
| Build Tools| xcode-select --install |
| Rust | curl --proto  '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh |

**Windows 10+**
| Dep  | Install Command                                                  |
|------|------------------------------------------------------------------|
| Chocolatey | [Installation Guide](https://chocolatey.org/install) |
| Rust | choco install rust |


---

# Things to fix / implement

We should try to finish this checklist before switching over to using this UI for the primary Uplink UI.

- [ ] Skeletal loaders for all relevant components inside the UIKit
  - [ ] User Image
  - [ ] Message
  - [ ] Button
  - [ ] Label
  - [ ] Select
  - [ ] File
  - [ ] Folder
  - [ ] User
  - [ ] Chat
  - [ ] Friend
- [ ] Toast Notifications
  - [ ] Ability to push a new toast notification.
  - [ ] Toast notification automatically dismisses after `n` seconds.
  - [ ] Hovering over toast notification should reset the dismiss timer.
  - [ ] Clicking the `x` on the toast notification should dismiss it immediatly.
- [ ] Calling Modal
  - [ ] Should be wired to state to appear when ui.incoming_call is set to some call.
  - [ ] We should outline a struct to neatly contain info we need pretaining to an incoming call.
- [ ] Files
  - [ ] Files should be able to be dragged and dropped into a folder in order to move the file into the folder.
  - [ ] We should be able to drag and drop to re-organize the files page
  - [ ] We should be able to rename folders
  - [ ] We should be able to drag and drop folders into folders.
  - [ ] We should be able to navigate using the breadcrumbs.
  - [ ] we should be able to delete files and folders. Deleting a folder should delete all the items inside a folder.
  - [ ] Deleting things should move them to a "trash" folder which will have a different icon. 
  - [ ] Emptying trash will delete everything in the trash.
- [ ] Language & Translation
  - [ ] Replace all references to plain text within the app to references to the translated items
  - [ ] Ensure that there are no hard coded text within the UIKit that we can't override with props. 
- [ ] Messaging
  - [ ] Add mock data to generate random replies to messages
  - [ ] Add mock data to generate random reactions to messages 
  - [ ] Add mock data to include random attachements on messages
  - [ ] Add ability to "edit" messages.
  - [ ] Implement UI for afformentioned items.
- [ ] Settings should be wired to config file and automatically update.
- [ ] CSS needs to be split up neater within components and layouts in uplink_skeleton.
- [ ] Unlock page needs porting.
- [ ] Account creation page needs porting.
- [ ] Add generic loader component.
- [ ] Add a config option to enable developer logging
  - [ ] Developer logging should write developer logs to uplink-debug.log
  - [ ] Include a way to view the contents of the log in developer settings.
    - [ ] Include a copy button to copy the log to clipboard.
  - [ ] Debug logging should also log neatly to rust console.
- [ ] Profile Page in settings
- [ ] Profile page popup option for user_image 