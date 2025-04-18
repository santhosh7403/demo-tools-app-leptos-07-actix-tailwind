<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>


This is a leptos demo application that will help anyone who wants a hands-on and learn by experimenting in a working example.
  
It includes:<br/>        
          Leptos<br/>
          actix-web<br/>
          SSR<br/>
          DB integration (sqlite)<br/>
          Modal Window<br/>


To test it out, clone the repo and run.

`git clone https://github.com/santhosh7403/demo-tools-app-leptos-07-actix-tailwind.git`

`cd demo-tools-app-leptos-07-actix-tailwind`

`cargo leptos watch`  or `cargo leptos serve`

This expects cargo-leptos is installed already, if you don't have `cargo-leptos` installed you can install it with

`cargo install cargo-leptos --locked`




By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate` etc. If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
3. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)

# Update(2025-04-18)

   This app build seems breaking with 1.88.0.nightly release, with below error:

                   error[E0599]: no method named `source_file` found for struct `memo::proc_macro::Span` in the current scope
                   --> /home/santhosh/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/leptos_macro-0.7.8/src/lib.rs:360:22
                    |
                360 |                 site.source_file().path(),
                    |                      ^^^^^^^^^^^
                    |
                help: there is a method `source` with a similar name
                    |
                360 -                 site.source_file().path(),
                360 +                 site.source().path(),

As a work around for now, adding a rust-toolchain.toml config file to force it to use 1.87.0 , at the project root folder.

You may need install it with rustup as below.

`rustup toolchain install nightly-2005-03-05-x86_64-unknown-linux-gnu`  - add this channel

`rustup target add wasm32-unknown-unknown`  - adding wasm target to the newly added channel

Now you may run the build.

`cargo leptos watch`  or `cargo leptos serve`


# Application access

Once application started, access application from you web browser [ localhost:3000 ](http://localhost:3000/)

Here are some screenshots.
![Screenshot from 2025-04-08 13-43-59](https://github.com/user-attachments/assets/8f50dc0d-694e-4e38-905e-196872baa6ef)
![Screenshot from 2025-04-07 13-29-26](https://github.com/user-attachments/assets/df187002-f0b5-44ed-8783-92fd5e122b5a)
![Screenshot from 2025-04-07 13-29-54](https://github.com/user-attachments/assets/38a5de8d-e3c7-4a6b-84c2-31d8f28617f5)
![Screenshot from 2025-04-07 13-28-38](https://github.com/user-attachments/assets/c3d5c0a3-fb14-4dcd-a08d-090e98840a2f)
![Screenshot from 2025-04-07 13-26-33](https://github.com/user-attachments/assets/4fafa405-d517-484c-b922-883c1f058cc5)

# Modal Window example:
![Screenshot from 2025-04-07 13-27-22](https://github.com/user-attachments/assets/1a018da0-1e0a-4a6a-b6bb-95390a9d5382)

# Normal edit form :

![Screenshot from 2025-04-08 14-24-09](https://github.com/user-attachments/assets/4a95fabe-a9f2-4e72-9e4c-f94d2fd799b1)


References and thanks:

1. [https://github.com/t4p-community/20240703_build-web-apps-with-rust-and-leptos] 

2. [https://github.com/oxide-byte/todo-leptos] 

This project is fork of the above (from 1) and in top of it following updates:<br/>
    leptos version 0.6 to 0.7 upgrade (breaking changes).<br/>
    tailwindcss applied.<br/>
    Modal window integrated (from 2).<br/>
