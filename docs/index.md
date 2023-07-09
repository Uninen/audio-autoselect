# Docs

## Background Of macOS Audio Output APIs

Weirdly enough, AFAIK, there is no public API for changing macOS System Audio Output. Core Audio has an API for retrieving the current output and a legacy `AudioDeviceSetProperty` method which has been deprecated for a long time. The current solutions that projects like [switchaudio-osx](https://github.com/deweller/switchaudio-osx) use rely on C code and these old APIs, and as of writing this in July 2023 they have [known issues with bluetooth devices](https://github.com/deweller/switchaudio-osx/issues/9) and newer versions of macOS. In other words, the macOS APIs around this are nonexistent and/or buggy.

A workaround for the poorly documented deprecated and buggy APIs is to use AppleScript to manually script interaction with the UI to accomplish the task. This does work but is obviously super hacky and brittle. I found after a weekend of experimenting that in practice a combination of these two methods would at least now be the best way to solve the issue.

For Web apps, there's also a third option, which is to use Web Audio APIs and `navigator.mediaDevices`. Perhaps not surprisingly, the support for output devices is not yet there with Tauri / Wry, so

### Current Quirks

After connecting a bluetooth headset and set it manually to be the default output, the device shows up correctly with both CoreAudio and Web APIs. But before interacting with the device manually (ie. setting it as output or input from the System Settings), bluetooth devices are completely invisible to both CoreAudio and Web APIs. It's not clear if this is a macOS bug or is it by design.

Again, this restriction can be worked around by using AppleScript. This feels like a very ugly hack, but it does work.
