<div align="center">
  <a href="https://github.com/OpenCz/wizo/">
    <img src="https://github.com/OpenCz/wizo/blob/main/assets/wizo-logo.png?raw=true" alt="Logo" height="180" style="border-radius: 10px">
  </a>

  <h3 align="center">wizo</h3>

  [![License](https://img.shields.io/github/license/OpenCz/wizo?style=for-the-badge)](./LICENSE)
  [![Build Status](https://img.shields.io/github/actions/workflow/status/OpenCz/wizo/ci.yml?style=for-the-badge)](https://github.com/OpenCz/wizo/actions)

  <p align="center">
    Light Self Hosted CI/CD runner.
    <br />
    <a href="https://github.com/OpenCz/wizo"><strong>Explore the repository »</strong></a>
    <br />
    <br />
    <a href="https://github.com/OpenCz/wizo">View Demo</a>
    &middot;
    <a href="https://github.com/OpenCz/wizo/issues/new?template=bug-report.yml">Report Bug</a>
    &middot;
    <a href="https://github.com/OpenCz/wizo/issues/new?template=feature-request.yml">Request Feature</a>
  </p>
</div>

<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>

## About The Project

wizo is a lightweight, self-hosted CI/CD tool implemented in Rust. It aims to provide a simple, extensible pipeline runner for small teams and personal projects that prefer a minimal, local-first continuous integration and deployment solution.

### Built With

[![Rust][Rust-shield]][Rust-url]


## Getting Started

To get a local copy running for development or testing, follow the steps below.

### Prerequisites

- Rust toolchain (`rustup`, `cargo`) installed (stable channel)
- A POSIX-like shell (Linux/macOS). Windows support via WSL or native builds.

### Installation

#### Development (build and run locally)
1. Clone the repository
```sh
git clone https://github.com/OpenCz/wizo.git
cd wizo
```
2. Build and run in development
```sh
cargo build
cargo run -- help
```

#### Release (build optimized binary)
```sh
cargo build --release
# run the produced binary
./target/release/wizo --help
```

## Usage

- See `--help` for CLI options and configuration flags after building the binary.
- Typical development flow:
  - Edit pipeline config files under the repository as needed.
  - Run `cargo run` during development to test changes.
  - Build with `cargo build --release` for production.

For more advanced deployment and configuration examples, check the `examples/` and `assets/` folders if present.

## Roadmap

- Add pipeline templates and example configs
- Improve plugin/runner integrations
- Add web UI for pipeline monitoring

See the [open issues](https://github.com/OpenCz/wizo/issues) for a full list of proposed features and known issues.

## Contributing

Contributions welcome — please follow the guidelines in [CONTRIBUTING.md](./CONTRIBUTING.md). Small, focused PRs with tests and clear descriptions are easiest to review.

### Top contributors:

<a href="https://github.com/OpenCz/wizo/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=OpenCz/wizo" alt="contrib.rocks image" />
</a>

## License

Distributed under the MIT License. See [LICENSE](./LICENSE) for more information.

## Contact

`@lukas-sgx` <lukas.soigneux@epitech.eu> (maintainer) — see the project repository for contact details and issue tracker.

<!-- ## Acknowledgments -->

[Rust-shield]: https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white
[Rust-url]: https://www.rust-lang.org/



