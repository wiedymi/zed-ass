# Development Guide for ASS Extension

## Prerequisites

Before developing this extension, ensure you have:

1. **Rust installed via rustup** - Required for building the extension
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Zed editor** - For testing the extension
   - Download from [zed.dev](https://zed.dev)

## Local Development

### Testing the Extension Locally

1. **Install as Dev Extension**:
   - Open Zed
   - Open the extensions panel (`Cmd+Shift+X` / `Ctrl+Shift+X`)
   - Click "Install Dev Extension"
   - Select this directory

2. **Test with Example File**:
   - Open `example.ass` in Zed
   - Verify syntax highlighting works
   - Check that the language server activates

### Building the Extension

```bash
# Build the Rust code
cargo build --release

# The compiled WebAssembly will be in target/wasm32-wasi/release/
```

## Publishing to Zed Extensions Registry

### Step 1: Create the Language Server and Grammar Repositories

Before publishing, you need to create:

1. **tree-sitter-ass** repository at `https://github.com/wiedymi/tree-sitter-ass`
   - Implement the Tree-sitter grammar for ASS files
   - Include proper query files for highlighting

2. **ass-lsp** repository at `https://github.com/wiedymi/ass-lsp`
   - Implement the Language Server Protocol for ASS files
   - Create GitHub releases with pre-compiled binaries

### Step 2: Update Extension Configuration

Once the repositories exist, update:

1. **extension.toml**: Verify the grammar repository URL and revision
2. **src/lib.rs**: Update the GitHub release download logic if needed

### Step 3: Fork and Submit to Extensions Registry

1. **Fork the extensions repository**:
   ```bash
   git clone https://github.com/YOUR_USERNAME/extensions
   cd extensions
   ```

2. **Add your extension as a submodule**:
   ```bash
   git submodule add https://github.com/wiedymi/zed-ass-extension.git extensions/ass
   git add extensions/ass
   ```

3. **Update extensions.toml**:
   ```toml
   [ass]
   submodule = "extensions/ass"
   version = "0.1.0"
   ```

4. **Sort extensions**:
   ```bash
   pnpm sort-extensions
   ```

5. **Submit Pull Request**:
   - Create a pull request to `zed-industries/extensions`
   - Include description of the extension functionality

## File Structure

```
zed-ass-extension/
├── extension.toml              # Extension manifest
├── Cargo.toml                  # Rust dependencies
├── src/
│   └── lib.rs                  # Extension implementation
├── languages/
│   └── ass/
│       ├── config.toml         # Language configuration
│       ├── highlights.scm      # Syntax highlighting queries
│       ├── brackets.scm        # Bracket matching
│       ├── outline.scm         # Document outline
│       └── indents.scm         # Indentation rules
├── example.ass                 # Example ASS file
├── README.md                   # Documentation
├── LICENSE                     # MIT License
└── .gitignore                  # Git ignore rules
```

## Language Server Integration

The extension automatically downloads and manages the ASS language server:

- Downloads from GitHub releases
- Caches binary for reuse
- Supports cross-platform binaries (Windows, macOS, Linux)
- Handles version updates automatically

## Tree-sitter Grammar Integration

The extension uses the external tree-sitter-ass grammar:

- Referenced in `extension.toml` 
- Downloaded and compiled by Zed automatically
- Provides accurate syntax parsing for ASS files

## Testing Checklist

Before publishing, ensure:

- [ ] Extension loads without errors
- [ ] Syntax highlighting works for all ASS constructs
- [ ] Language server downloads and starts correctly
- [ ] File association works for `.ass` and `.ssa` files
- [ ] Outline view shows proper structure
- [ ] Bracket matching works correctly
- [ ] No console errors in Zed

## Troubleshooting

### Extension Not Loading

1. Check Zed console for errors
2. Verify Rust is installed via rustup
3. Ensure all required repositories exist

### Language Server Not Starting

1. Check network connectivity
2. Verify GitHub release exists with proper binary names
3. Check Zed's language server logs

### Syntax Highlighting Not Working

1. Verify tree-sitter grammar repository is accessible
2. Check query files for syntax errors
3. Ensure grammar builds successfully

## Contributing

1. Fork this repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

For questions or support, contact: contact@wiedymi.com