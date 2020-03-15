hello_rustc demonstrates how to interact with the Rust compiler using rustc_interface.

To build, you will need to switch to rustc nightly and add the `rustc-dev` component:

    rustup default nightly
    rustup component add --toolchain nightly rustc-dev

After doing this, `cargo run` should work.