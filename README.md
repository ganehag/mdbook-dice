# mdbook-dice Preprocessor

The `mdbook-dice` preprocessor is an extension for [mdBook](https://github.com/rust-lang/mdBook) designed to enhance tabletop RPG (TTRPG) manuals and guides written with mdBook. It introduces custom dice notation, allowing authors to specify dice rolls directly in their Markdown content, which are then automatically formatted and stylized in the generated book.

## Features

- **Custom Dice Notation**: Authors can use a simple syntax to indicate dice rolls, including modifiers for advantage and disadvantage.
- **Localization Support**: Recognizes dice notation in multiple languages, adapting to the word for "dice" in each language (e.g., D20 in English, T20 in Swedish).
- **Flexible Parsing**: Handles optional quantity specifications and various dice types, ensuring compatibility with a wide range of tabletop games.
- **Customizable Visuals**: Generates HTML spans with class attributes for each dice roll, allowing for extensive visual customization through CSS.

## Installation

1. Ensure you have [mdBook](https://github.com/rust-lang/mdBook) installed.
2. Clone this repository to your local machine.
3. Build the `mdbook-dice` preprocessor using Cargo:
   ```
   cargo build --release
   ```
4. The executable will be located in `target/release/`. Add it to your PATH or reference it directly in your `book.toml`.

## Usage

### Configuring Your mdBook

To use the `mdbook-dice` preprocessor in your mdBook project, add the following lines to your `book.toml`:

```toml
[preprocessor.dice]
command = "path/to/mdbook-dice"
```

Replace `path/to/mdbook-dice` with the actual path to the `mdbook-dice` binary.

### Writing Dice Notation

- **Normal Rolls**: `[[1D20]]` or `[[D20]]` for a single 20-sided dice roll.
- **Advantage Rolls**: `[[+1D20]]` for rolling with advantage.
- **Disadvantage Rolls**: `[[-1D20]]` for rolling with disadvantage.

### Styling the Output

The preprocessor wraps each dice notation in an HTML `<span>` element with class attributes indicating the type of roll (e.g., `dice-roll`, `dice-roll advantage`, `dice-roll disadvantage`). To style these elements, include the following in your mdBook's CSS:

```css
span.dice-roll {
    /* Basic dice roll styling */
}

span.dice-roll.advantage {
    /* Styling for advantage rolls */
}

span.dice-roll.disadvantage {
    /* Styling for disadvantage rolls */
}
```

Customize the CSS according to your design preferences.

## Contributing

Contributions to `mdbook-dice` are welcome! Whether it's adding new features, improving existing ones, or reporting bugs, please feel free to open an issue or submit a pull request.

## License

`mdbook-dice` is licensed under [MIT](LICENSE) or any other license specified in the LICENSE file.

