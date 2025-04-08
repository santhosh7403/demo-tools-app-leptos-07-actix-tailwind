<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>


This is a leptos demo application that will help you to experience and learn those who are interested.

To test it out, clone the repo and run.

`git clone https://github.com/santhosh7403/demo-tools-app-leptos-07-actix-tailwind.git`

`cd demo-tools-app-leptos-07-actix-tailwind`

`cargo leptos watch`

This expects cargo-leptos is installed already, if you don't have `cargo-leptos` installed you can install it with

`cargo install cargo-leptos --locked`




By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate` etc. If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
3. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)


