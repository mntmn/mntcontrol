# mntcontrol

A minimalist web control panel for zigbee2mqtt things (currently: lights). 

## Configuration

Copy `mntconfig.toml.default` to `mntconfig.toml`.

`bind` is the address and port for the web server of mntcontrol.
The `mqtt` table controls `host` and `port` of the MQTT server (tested with zigbee2mqtt).

For each light that you want to control (currently only `brightness` can be set), duplicate the section:
```
[[lights]]
name = "0x7cb03eaa00a47bee"
title = "Lab Tube 1"
```

## Running

```
cargo run
```

## Using

Point a (mobile) browser to the address configured as `bind`. Then click buttons to control lights.

## License

The license of this project is GPLv3.
