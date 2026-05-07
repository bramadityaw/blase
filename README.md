# (B)lade (LA)nguage (SE)rver

Language server for Laravel's Blade templating language.

🚨 WARNING: As of right now, Blase does NOT provide services for components and directives from packages.

## Table of Contents

- [Installation](#installation)
  - [VS Code](#vs-code)
  - [Neovim](#neovim)
- [Features](#features)

# Installation

Download the binary from the latest release and add it to your PATH.

## VS Code

- Download the extension (`.vsix`) from the latest release.

From the Extensions view in VS Code:
- Go to the Extensions view.
- Select Views and More Actions...
- Select Install from VSIX...

Or from the command line:

```bash 
# if you use VS Code
code --install-extension blase.vsix

# if you use VS Code Insiders
code-insiders --install-extension blase.vsix
```

## Neovim

- Neovim >= 0.11

```lua
vim.lsp.config.blase = {
    cmd = { 'blase' },
    filetypes = { 'blade' },
    root_markers = { 'artisan', 'composer.json', '.git' },
}
vim.lsp.enable 'blase'
```

# Features
-  **Go To Definition**: Jump to the file where a component or layout is defined
-  **Find References**: Find all usages of a component or layout
-  **Hover**: View available attributes and documentation of components & layouts
-  **Completion**:
    =  Directives
        -  Control flow (@if, @while, ...)
        -  Element dependent inline attributes (@checked, @selected, etc.)
    -  Component/Layout
        -  Component Name
        -  Layout Name
        -  Attributes
-  **Signature Help**: Component attribute assistance
-  **Diagnostic**:
    -  Syntax errors
    -  Unknown component/layout
-  **Workspace Symbols**: Search all available components and layouts in the project
