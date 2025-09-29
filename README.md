# Gradient Image Generator

This is a simple Rust project that generates an image with a gradient and saves it as `gradient.png`.

![Gradient Image](./gradient.png)

## Getting Started

### Prerequisites

*   Rust and Cargo must be installed on your system.

### Installation

1.  Clone the repository:
    ```bash
    git clone <repository-url>
    ```
2.  Navigate to the project directory:
    ```bash
    cd gradient
    ```

## Usage

To build and run the project, execute the following command:

```bash
cargo run
```

This will generate a `gradient.png` file in the root of the project directory.

## Dependencies

This project uses the following dependencies:

*   `image`
*   `imageproc`
*   `num-complex`

*Note: The `imageproc` and `num-complex` dependencies are not used in the current version of the `main.rs` file, but are included in the project.*
