# Email Validation Library

This Rust library provides functionality for validating email addresses based on different criteria,
such as whether they belong to free or disposable/temporary email domains.
The library offers features that can be toggled using Rust Cargo features.

## Features

The library provides the following functions.

`is_work_domain`: Checks if the given domain is a work email domain.
`is_work_email`: Checks if the given email address belongs to a work email domain.

These two functions will check depends on the features you enabled,
if you enabled the `free` feature, it will check if the email is not a free email,
if you enabled the `disposable` feature, it will check if the email is not a disposable email.
And if you enabled both, it will check if the email is not a free email and not a disposable email.

### Free Email Validation

Enable the `free` feature to include functionality related to free email validation, e.g: `gmail.com`.
The module includes a pre-generated list of free email domains, and the following functions are provided:

- `is_free_domain`: Checks if the given domain is a free email domain.
- `is_free_email`: Checks if the given email address belongs to a free email domain.

### Disposable Email Validation

Enable the "disposable" feature to include functionality related to disposable/temporary email validation, e.g: `temp-mail.org`. 
The module includes a pre-generated list of disposable email domains, and the following functions are provided:

- `is_disposable_domain`: Checks if the given domain is a disposable email domain.
- `is_disposable_email`: Checks if the given email address belongs to a disposable email domain.

This feature is disabled by default, because the list of disposable email domains is large and may not be necessary for all use cases.

## Usage

To use this library, add it as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
email_validation = "0.1"
```

The default feature is set to `free`, meaning that the free email validation functions are included by default.
You can customize the features by specifying them in the `Cargo.toml` file.

### Example Usage

```rust
use email_validation::{is_free_email, is_disposable_email};

fn main() {
    let email = b"test@gmail.com";
    
    if is_free_email(email) {
        println!("This is a free email address.");
    } else {
        println!("This is not a free email address.");
    }

    if is_disposable_email(email) {
        println!("This is a disposable email address.");
    } else {
        println!("This is not a disposable email address.");
    }
}
```

## Disclaimer

This library is not a complete solution for email validation, and it is not guaranteed to be 100% accurate.

It is intended to be used as a tool to help filter out free email and disposable email addresses providers.

If you need to also validate the format of the email address, you should use other libraries, like [email_address](https://crates.io/crates/email_address) or [fast_chemail](https://crates.io/crates/fast_chemail/) or tools in combination with this library.

## Contributing

Feel free to contribute to the project by opening issues or submitting pull requests. Ensure that any new features or changes are thoroughly tested.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
