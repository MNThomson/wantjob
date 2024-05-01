# WantJob

[<img alt="github" src="https://img.shields.io/badge/github-MNThomson/wantjob-bc3f48?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/MNThomson/wantjob)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/MNThomson/wantjob/ci.yml?branch=master&style=for-the-badge&logo=githubactions&logoColor=white" height="20">](https://github.com/MNThomson/wantjob/actions?query=branch%3Amaster)

## Introduction

## Usage

## Development

wantjob can be run locally for either further development or customization.

> [!NOTE]
> **BEFORE** you run the following steps make sure you have (a recent version of) Rust installed locally on you machine ```rustup update && rustup install nightly```

```shell
# 1. Clone the repository
git clone https://github.com/MNThomson/wantjob.git && cd wantjob

# 2.1 To start developing, run wantjob
cargo run

# 2.2 Or auto rebuild/run on file save (requires the mold linker & cranelift backend)
cargo r
```

The development environment is now running and accesible at https://localhost:4321/

#### License

<sup>
Licensed under <a href="LICENSE">AGPLv3</a>
</sup>
<br>
<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you shall be licensed as above, without any additional terms or conditions
</sub>
