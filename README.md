# NMEA-0183
A nmea-0183 sentence parser written in Rust using Nom.

<p align="center">
  <a href="https://crates.io/crates/nmea-0183">
      <img src="https://meritbadge.herokuapp.com/nmea-0183" alt="crates.io/nmea-0183">
  </a>
  <a href="https://travis-ci.com/YellowInnovation/nmea-0183">
      <img src="https://img.shields.io/travis/YellowInnovation/nmea-0183/master.svg" alt="Travis Build Status">
  </a>
  <a href="https://docs.rs/nmea-0183/">
      <img src="https://docs.rs/nmea-0183/badge.svg" alt="documentation">
  </a>
</p>


## Example

```rust
use nmea_0183::sentence::parse;

fn main() {
    // We first need a sentence to parse.
    // According to the NMEA-0183 specification, a sentence ends with <CR><LF>
    let raw_sentence = "$GPGGA,092725.00,4717.11399,N,00833.91590,E,1,08,1.01,499.6,M,48.0,M,,*5B\r\n";
    let parsed_sentence = parse(raw_sentence)
        .expect("Could not parse nmea sentence.");

    println!("{:?}", parsed_sentence);

    /*
    Sentence {
        sentence_type: Parametric,
        talker: GPS,
        message: GGA(GGAMessage {
            time: Some(09:27:25),
            lat: Some(Degree(47.1711399)),
            ns: North,
            lon: Some(Degree(8.339159)),
            ew: East,
            quality: AutonomousGNSSFix,
            num_sv: Some(8),
            hdop: Some(1.01),
            alt: Some(Meter(499.6)),
            sep: Some(Meter(48.0)),
            diff_age: None,
            diff_station: None
        })
    }
    */

}
```

## Status

The parser is at an early stage, I have written it by following the [U-Blox Receiver Protcol Specification](https://www.u-blox.com/sites/default/files/products/documents/u-blox8-M8_ReceiverDescrProtSpec_%28UBX-13003221%29_Public.pdf) and interpreting it as well as I could.

I have written unit tests based on the provided samples, but I don't have any receiver yet to test the output.

If you would like to improve it, or if you find a (one of many!) bug please [open an issue](https://github.com/YellowInnovation/nmea-0183/issues/new) or, even better, [submit a pull request](https://github.com/YellowInnovation/nmea-0183/compare) :)

## How to install

If you use [cargo-edit](https://github.com/killercup/cargo-edit) (which I recommend), open a shell in the project you want to add the library to and run the following command:

```bash
$ cargo add nmea-0183
      Adding nmea-0183 v0.0.2 to dependencies
```

If you don't, add the library to your Cargo.toml dependencies:

```toml

[dependencies]
nmea-0183 = "*"

```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Dual MIT/Apache2 is strictly more permissive
