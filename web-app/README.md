# img_stegano Web App

A modern web application for hiding and revealing secret messages in images using steganography.

## Features

- **Encode**: Hide secret messages in PNG and JPEG images
- **Decode**: Extract hidden messages from images
- **Capacity Indicator**: Shows how much data can be hidden in an image
- **Modern UI**: Built with Tailwind CSS and React
- **Fast**: Powered by WebAssembly for near-native performance

## Tech Stack

- **React** + **TypeScript** - UI framework
- **Vite** - Build tool
- **Tailwind CSS** - Styling
- **WebAssembly** - Core steganography engine (Rust)
- **react-dropzone** - File upload

## Development

```bash
# Install dependencies
yarn install

# Start dev server (automatically builds WASM if needed)
yarn dev

# Build for production (includes WASM build)
yarn build

# Preview production build
yarn preview
```

## How It Works

The app uses LSB (Least Significant Bit) steganography to hide messages in the RGB channels of image pixels. Each pixel can store 3 bits of data (1 bit per color channel), making the changes imperceptible to the human eye.

**Important**: Use lossless formats like PNG for best results. JPEG compression may corrupt hidden messages.

## Project Structure

```
web-app/
├── src/
│   ├── App.tsx           # Main application component
│   ├── main.tsx          # Entry point
│   ├── index.css         # Tailwind CSS imports
│   └── utils/
│       └── files.ts      # File handling utilities
├── pkg/                  # WASM module (generated)
└── dist/                 # Production build (generated)
```
