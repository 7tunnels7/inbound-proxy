# Rust Inbound Proxy

The Rust Inbound Proxy is a lightweight and efficient proxy server written in Rust. It provides a seamless way to proxy inbound network traffic to a specified target host and port. With its simple command-line interface and robust functionality, it offers a reliable solution for transparently forwarding traffic.

## Features

- Inbound proxy: Listens for incoming connections and forwards traffic to the target host and port.
- Multi-threaded handling: Uses separate threads to handle multiple client connections simultaneously, ensuring optimal performance.
- Real-time logging: Displays detailed logs about received and sent traffic, including timestamps and data sizes.
- Command-line configuration: Accepts command-line arguments for easy customization of the target host, target port, and listener port.

## Usage

To run the Rust Inbound Proxy, use the following command:

```shell
cargo run -- --target-host <target_host> --target-port <target_port> --listener-port <listener_port>
```

Replace `<target_host>`, `<target_port>`, and `<listener_port>` with the desired values:

- `<target_host>`: The host address to which traffic will be forwarded.
- `<target_port>`: The port on the target host where the traffic will be sent.
- `<listener_port>`: The port on the local machine where the proxy server will listen for incoming connections.

Example:

```shell
cargo run -- --target-host localhost --target-port 22 --listener-port 6969
```

Once the proxy server is running, it will display logs indicating the successful initialization and the traffic forwarding process.

## Dependencies

The Rust Inbound Proxy relies on the following external dependencies:

- `clap`: A command-line argument parsing library for parsing and validating command-line arguments.
- `colored`: A library for adding colors and styles to the terminal output.

## Conclusion

The Rust Inbound Proxy offers a reliable and performant solution for proxying inbound network traffic. With its easy-to-use command-line interface and robust functionality, it provides a seamless experience for transparently forwarding traffic to a specified target host and port. Enjoy the power and efficiency of Rust with this lightweight proxy server! 🚀
