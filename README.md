# Sensor API

## Build from source

1. Install rust

    https://www.rust-lang.org/en-US/install.html

2. Install rust nightly

        $ rustup install nightly

3. Set nightly as default toolchain

        $ rustup default nightly

4. (Optional) Modify *Rocket.toml* to set API address

5. (Optional) Modify *Nodelink.toml* to configure link between (mbed_node or mock_node) and api listener.

4. Run
    
        $ cargo run --bin server --release

5. (Optional) To run without mbed fixed node, run the mock node which will simulate it.

        $ cargo run --bin mock_node --release

## API Usage

View API usage documentation [here](https://documenter.getpostman.com/view/5796702/RzZ6K1X7).
