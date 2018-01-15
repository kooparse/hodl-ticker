# cryptocurrency-rs

## Screenshot
<img width="682" alt="screen shot 2017-12-28 at 12 22 02 pm" src="https://user-images.githubusercontent.com/2859122/34409421-07aa0a36-ebca-11e7-80e6-6c50d63e960a.png">


## Install && Running
- Fork or download this repository.
- `cd` to the project's location
- run `cargo build --release`
- run `./target/release/cryptocurrency-rs -h`

## Options
You can use the `-c` (or `--convert`) with the fiat currency symbol to find in terms of another currency.
The default currency is USD and it supports AUD, BRL, CAD, CHF, CLP, CNY, CZK, DKK, EUR, GBP, HKD, HUF, IDR, ILS, INR, JPY, KRW, MXN, MYR, NOK, NZD, PHP, PKR, PLN, RUB, SEK, SGD, THB, TRY, TWD, ZAR.

```
// Convert prices to Euro
$ cryptocurrency-rs -c eur
// Convert prices to Yenn
$ cryptorrency-rs -c jpy
```

You can use the `-d` (or `--desired`) to choose which cryptocurrencies to see.
```
$ cryptocurrency-rs -d Bitcoin "Bitcoin Cash"
```

You can use the `-h` (or `--help`) to find all valid options of cryptocurrency-rs


This project is highly inspired by coinmon: https://github.com/bichenkk/coinmon
