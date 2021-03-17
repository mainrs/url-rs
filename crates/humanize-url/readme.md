# humanize-url

> A library for creating human-readable URLs.

## Example

```rust
use humanize_url::humanize_url;

let url = humanize_url("https://github.com/SirWindfield").unwrap();
assert_eq!("github.com/SirWindfield", url);
```

## Use-case

I use the library often when working with links inside of terminals. Together with [terminal-link](https://github.com/SirWindfield/terminal-link-rs) it can be used to print prettier links to the terminal.
