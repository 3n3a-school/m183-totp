# m183-totp

> TOTP example application in rust's rocket

## Start

To start the application you need docker and
docker-compose.

Then just clone this repo / download `docker-compose.yml`
Afterwards run the following:

```bash
docker-compose up -d
```

The application should now be available at [http://localhost:8007](http://localhost:8007/).

## Architecture

* rocket (rocket.rs - Webframework )
* rocket_auth (+ some fixes by me)
* google_authenticator (library for totp)
