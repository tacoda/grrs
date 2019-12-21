# grrs

> A grep copy implemented in Rust

## Install

```sh
$ cargo install tacoda_grrs
```

## Usage

```sh
$ tacoda_grrs foo test.txt
```

## Notes

### Releasing on GitHub

Travis build scripts from [Trust repo](https://github.com/japaric/trust)

Push a tag to trigger a release build for multiple operating systems
and create a release on GitHub.

### Releasing to Cloud Storage

Add config to Travis yaml file.

### Distributing with Cargo

Make package info updates in `Cargo.toml`

```sh
$ cargo login
$ cargo publish
```

### Distributing with Trust

[Trust repo](https://github.com/japaric/trust)

```sh
curl -LSfs https://japaric.github.io/trust/install.sh | \
    sh -s -- --git tacoda/grrs
```

### Distributing with NPM

`install.js`:

```javascript
let exec = require('child_process').exec;

exec('curl -LSfs https://japaric.github.io/trust/install.sh | \
sh -s -- --git tacoda/grrs', (error, stdout, stderr) => {
  console.log(stderr);
});
```

`package.json`:

```javascript
{
    "...": "...",
    "postinstall": "npm run install",
    "scripts": {
        "install": "node install.js"
    },
    "...": "..."
}
```

```sh
$ npm install -g grrs
```
### Distributing with Brew

[Brewfile example from ripgrep](https://github.com/BurntSushi/ripgrep/blob/31adff6f3c4bfefc9e77df40871f2989443e6827/pkg/brew/ripgrep-bin.rb)
