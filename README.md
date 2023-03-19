
# Rust Clean

Rust Clean Architecture project.

Please see [Uncle Bob article](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html) or [some of these visuals](https://proandroiddev.com/clean-architecture-data-flow-dependency-rule-615ffdd79e29) for more information.



## Clean Architecture
![Clean Architecture](docs/CleanArchitecture.jpg)


Dependencies should be Outside -> In. This means that no inner circle can depend on any of the outer circles.

In this folder structure we have:
- Infrastructure
- Presentation
- Application
- Domain

Both **Infrastructure** and **Presentation** are the top external layers (`Blue` and `Green` areas in the above picture).

`Infrastructure` contains any persistent logic and external providers.

`Presentation` contains the `dtos` and a `rest_api` to interact with `Application`.

`Application` is the application logic, it constains the business and use cases that interact with our `Domain`.

`Domain` includes all entities that form our internal domain.


## Commands

```rust
./scripts/run_postgres.sh // initial database setup. It runs docker base image and seeds the database.

cargo build // builds the app
cargo run // runs the app
```