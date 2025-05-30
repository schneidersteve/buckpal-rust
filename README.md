# Rust Example Implementation of a Hexagonal/Onion/Clean Architecture

Inspired by https://github.com/thombergs/buckpal

- Kotlin Version: https://github.com/schneidersteve/buckpal-kotlin
- Dart Version: https://github.com/schneidersteve/buckpal-dart

## Tech Stack

* [Rust](https://www.rust-lang.org)
* [Rust Async](https://rust-lang.github.io/async-book/)
* [Mockall](https://github.com/asomers/mockall)
* [Salvo](https://salvo.rs)
* [openssl](https://docs.rs/openssl/latest/openssl/)
* [SQLx](https://github.com/launchbadge/sqlx)
* [rust-analyzer](https://rust-analyzer.github.io)
* [Visual Studio Code](https://code.visualstudio.com)
* [Visual Studio Code Dev Containers](https://code.visualstudio.com/docs/devcontainers/containers#_quick-start-open-a-git-repository-or-github-pr-in-an-isolated-container-volume)

## Layers and Dependency Inversion

![Dependency Inversion](di.png)

## Send Money Use Case

```gherkin
Feature: Send Money

  Scenario: Transaction succeeds
    Given a source account
    And a target account

    When money is send

    Then send money succeeds

    And source account is locked
    And source account withdrawal will succeed
    And source account is released

    And target account is locked
    And target account deposit will succeed
    And target account is released

    And accounts have been updated
```

# Cargo Examples

> cargo clean

> cargo build

> cargo test

> cargo run

> cargo update

> cargo install cargo-watch

> cargo watch --clear -x "run"

> cargo watch --clear -x "test"

> cargo watch --clear -x "test -- --show-output"
