# ASS Subtitle Format Extension for Zed

This extension provides language support for ASS (Advanced SSA) subtitle files in the Zed editor.

## Features

- **Syntax Highlighting**: Full syntax highlighting for ASS subtitle format including:
  - Section headers (`[Script Info]`, `[V4+ Styles]`, `[Events]`)
  - Key-value pairs in configuration sections
  - Style definitions and properties
  - Dialogue events and timing
  - Style override tags
  - Comments and metadata

- **Language Server**: Integration with the ASS Language Server for:
  - Real-time error checking
  - Auto-completion
  - Format validation
  - Style suggestions

- **File Association**: Automatic detection of `.ass` and `.ssa` files

## Installation

1. Open the Zed extensions panel (`Cmd+Shift+X` / `Ctrl+Shift+X`)
2. Search for "ASS Subtitle Format"
3. Click Install

## Usage

Simply open any `.ass` or `.ssa` file in Zed and the extension will automatically activate, providing syntax highlighting and language server features.

## ASS File Format

The ASS (Advanced SubStation Alpha) format is a subtitle format that supports:

- Rich text formatting (bold, italic, colors)
- Positioning and animation
- Multiple subtitle tracks
- Custom styling and fonts

Example ASS file structure:

```
[Script Info]
Title: Example Subtitles
ScriptType: v4.00+

[V4+ Styles]
Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding
Style: Default,Arial,20,&H00FFFFFF,&H000000FF,&H00000000,&H80000000,0,0,0,0,100,100,0,0,1,0,0,2,0,0,0,1

[Events]
Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text
Dialogue: 0,0:00:01.00,0:00:05.00,Default,,0,0,0,,Hello, world!
```

## Language Server

This extension uses the [ass-lsp](https://github.com/wiedymi/ass-lsp) language server to provide enhanced editing features. The language server is automatically downloaded and managed by the extension.

## Tree-sitter Grammar

Syntax highlighting is powered by the [tree-sitter-ass](https://github.com/wiedymi/tree-sitter-ass) grammar, which provides accurate parsing of ASS subtitle files.

## Development

This extension was developed using the Zed extension API. For more information about developing Zed extensions, see the [official documentation](https://zed.dev/docs/extensions).

## Author

- **wiedymi** - contact@wiedymi.com

## License

This project is licensed under the MIT License.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests to improve the extension.