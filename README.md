# earthquaked
`earthquaked` is an earthquake monitoring daemon that runs commands you specify when Early Earthquake Warning is annouced.

# Usage
```sh
$ earthquaked
```

# Installation
## Building from source
With cargo, you can install `earthquaked` by executing:
```sh
$ cargo install earthquaked
```

# Configuration
`earthquaked` searches `$HOME/.config/earthquaked/earthquaked.yaml` for a configuration file.
The example is available [here](./earthquaked.yaml)