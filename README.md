# Timezone Rest API

## Description

Small webserver that can deliver timezone information for geo coordinates. For example, you can request this resource:

```
http://localhost:8080/api?lng=52.517932&lat=13.402992
```

And, the service will respond with a json:

```json
{
  "name": "Europe/Berlin",
  "id": "CEST",
  "offset": 7200
}
```

`name` is the common name for the timezone. `id` is the short identifier. `offset` is the difference in seconds to UTC.

## How to use

This project based on rust. To run it you have to install rust and cargo and then you can use it with `RUST_LOG=info cargo run`. Thats all. Or you can use the prebuild docker container. 

### Docker

```
docker run -p8080:8080 racemap/rust-tz-service
```

If you want to build the container yourself, change to the project folder and run `docker build -t rust-tz-service .`. To start the container after build run `docker run -p 8080:8080 rust-tz-service`.

## Thanks to

This project use that great rust crate to get the timezones: [ringsaturn/tzf-rs](https://crates.io/crates/tzf-rs).