# Noise protocol framework in rust

![Build](https://github.com/Cyphernet-WG/rust-cyphernet/workflows/Build/badge.svg)
![Tests](https://github.com/Cyphernet-WG/rust-cyphernet/workflows/Tests/badge.svg)
![Lints](https://github.com/Cyphernet-WG/rust-cyphernet/workflows/Lints/badge.svg)
[![codecov](https://codecov.io/gh/Cyphernet-WG/rust-cyphernet/branch/master/graph/badge.svg)](https://codecov.io/gh/Cyphernet-WG/rust-cyphernet)

[![crates.io](https://img.shields.io/crates/v/noise-framework)](https://crates.io/crates/noise-framework)
[![Docs](https://docs.rs/noise-framework/badge.svg)](https://docs.rs/noise-framework)
[![Apache-2 licensed](https://img.shields.io/crates/l/noise-framework)](./LICENSE)


## Overview

The library provides pure rust implementation of 
[Noise protocol framework](http://noiseprotocol.org/) with minimal dependencies
made in functional style, where a specific Noise scheme is constructed as a
concrete type (like `Noise_XK<Secp256,ChaChaPoly,Sha256>`). Currently, the 
library supports following handshake patterns and construction primitives:
- `NN`
- `XK`
- `ChaChaPoly`
- `Secp256k1`
- `25519`
- `Sha256`
- `Sha3/256`
- `Blacke3`

The library is a part of [rust cyphernet suite](https://github.com/Cyphernet-WG/rust-cyphernet).


## Manifest

```yaml
Name: noise-framework
Type: Library
Kind: Free software
License: Apache-2.0
Language: Rust
Compiler: 1.65
Author: Maxim Orlovsky
Maintained: Cyphernet Initiative, Switzerland
Maintainers:
  Maxim Orlovsky:
    GitHub: @dr-orlovsky
    GPG: EAE730CEC0C663763F028A5860094BAF18A26EC9
    SSH: BoSGFzbyOKC7Jm28MJElFboGepihCpHop60nS8OoG/A
    EMail: dr@orlovsky.ch
  Alexis Sellier:
    GitHub: @cloudhead
    SSH: iTDjRHSIaoL8dpHbQ0mv+y0IQqPufGl2hQwk4TbXFlw
```


## Documentation

API reference documentation for the library can be accessed at
<https://docs.rs/noise-framework/>.


## Licensing

The libraries are distributed on the terms of Apache 2.0 opensource license.
See [LICENCE](LICENSE) file for the license details.