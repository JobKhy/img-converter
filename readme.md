# Image Converter

Image Converter is a high-performance image conversion library written in Rust and exposed to Node.js using `napi-rs`. It allows you to convert images between multiple formats efficiently, either from file paths or buffers.

## Features

- 🚀 Fast image conversion powered by Rust
- 🎨 Supports multiple formats: **PNG, JPEG, BMP, GIF, TIFF**
- 🔧 Simple API for file-based and buffer-based conversions
- 🛠️ Built with `napi-rs` for seamless Node.js integration

## Installation

<!-- Ensure you have **Node.js** and **Rust** installed. -->

```sh
npm install @jobkhy/image-converter
```

## Usage

### CommonJS

```js
const { convertImage, convertImageFile } = require("image-converter");

// Convert an image file
convertImage("input.jpg", "output.png", "png");

// Convert an image buffer
const fs = require("fs");
const inputBuffer = fs.readFileSync("input.jpg");
const outputBuffer = convertImageFile(inputBuffer, "png");
fs.writeFileSync("output.png", outputBuffer);
```

### ES Modules

```js
import { convertImage, convertImageFile } from "image-converter";

await convertImage("input.jpg", "output.png", "png");

import fs from "fs";
const inputBuffer = fs.readFileSync("input.jpg");
const outputBuffer = convertImageFile(inputBuffer, "png");
fs.writeFileSync("output.png", outputBuffer);
```

## API

### `convertImage(inputPath: string, outputPath: string, format: string): Promise<void>`

Converts an image file to the specified format and saves it to disk.

### `convertImageFile(inputData: Buffer, format: string): Buffer`

Converts an image buffer to the specified format and returns a new buffer.

## Supported Formats

- PNG
- JPEG / JPG
- BMP
- GIF
- TIFF
- WEBP

## Requirements

<!-- - Rust and `cargo` installed -->

- Node.js 16+ (with N-API support)

## License

MIT
