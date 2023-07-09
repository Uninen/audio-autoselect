# Audio Autoselect

A simple macOS program for autoselecting desired audio interface as default input / output as soon as it's connected.

I needed this because macOS often doesn't set my Bluetooth Beats Fit Pros as default device when I connect them so every time I jump into a Google Meet or Slack Huddle I need to manually first open up System Settings, select Audio tab, then select the correct input/output, and then refresh the browser window. As long as this app is running, the defaults are set automatically every time I connect the audio device. [See docs](docs/index.md) for details.

This project is in very aplha status. YMMV.

## TODO

- System tray window
- Remember user preferences
- Initial setup mode w/ docs
- App icon
- Tests

## Developing

- Install [Tauri dependencies](https://tauri.app/v1/guides/getting-started/prerequisites)
- Install frontend dependencies: `pnpm i`
- Run the dev server: `pnpm dev`

## Building Locally

- Install Tauri dependencies
- Run `pnpm install` and `pnpm build`

## Elsewhere

- [Follow @uninen](https://twitter.com/uninen) on Twitter
- Read my continuously updating learnings around Tauri / Vue / TypeScript and other Web development topics from my [Today I Learned site](https://til.unessa.net/)

## Contributing

Contributions are welcome! Please follow the [code of conduct](./CODE_OF_CONDUCT.md) when interacting with others.
