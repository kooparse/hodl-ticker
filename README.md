# Crypto-rs

## Screenshot
<img width="798" alt="screen shot 2017-12-09 at 2 24 25 pm" src="https://user-images.githubusercontent.com/2859122/33796036-05b7b7c8-dced-11e7-96a9-8637e00e6cb0.png">


## Install && Running
- Fork or download this repository.
- `cd` to the project's location
- run `cargo build --release`
- run `./target/release/rust-crypto -h`

## Options
You can use the `-c` (or `--convert`) with the fiat currency symbol to find in terms of another currency.
The default currency is USD and it supports AUD, BRL, CAD, CHF, CLP, CNY, CZK, DKK, EUR, GBP, HKD, HUF, IDR, ILS, INR, JPY, KRW, MXN, MYR, NOK, NZD, PHP, PKR, PLN, RUB, SEK, SGD, THB, TRY, TWD, ZAR.

```
// Convert prices to Euro
$ crypto-rs -c eur
// Convert prices to Yenn
$ crypto-rs -c jpy
```

You can use the `-h` (or `--help`) to find all valid options of crypto-rs


This project is highly inspired by coinmon: https://github.com/bichenkk/coinmon
