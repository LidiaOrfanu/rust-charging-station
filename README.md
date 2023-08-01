# rust-charging-station

Backend deployed using fly.io: <https://rust-charging-station.fly.dev>
Use example: <https://rust-charging-station.fly.dev/hello>

Makefile with start server cmd: quietly, clear (between each recompile) watch only the src folder -execute run
cargo watch -q -c -w src/ -x run

## Model

ChargingStation:
    - id: i32,
    - name: String,
    - location: String,
    - availability: bool

## Contributors

- Donna
- Luc
- Atte
- Lidia
- Marcus
