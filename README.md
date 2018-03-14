# HODL-TICKER

## Screenshot
<img width="682" alt="screen shot 2017-12-28 at 12 22 02 pm" src="https://user-images.githubusercontent.com/2859122/34409421-07aa0a36-ebca-11e7-80e6-6c50d63e960a.png">

## Download binary
- `cargo install hodl-ticker`

## Install && Running
- Fork or download this repository.
- `cd` to the project's location
- run `cargo build --release`
- run `./target/release/hodl-ticker -h`

## Options
You can use the `-c` (or `--convert`) with the fiat currency symbol to find in terms of another currency.
The default currency is USD and it supports AUD, BRL, CAD, CHF, CLP, CNY, CZK, DKK, EUR, GBP, HKD, HUF, IDR, ILS, INR, JPY, KRW, MXN, MYR, NOK, NZD, PHP, PKR, PLN, RUB, SEK, SGD, THB, TRY, TWD, ZAR.

```
// Convert prices to Euro
$ hodl-ticker -c eur
// Convert prices to Yenn
$ cryptorrency-rs -c jpy
```

You can use the `-f` (or `--filter`) to choose which cryptocurrencies to see.
```
$ hodl-ticker -f Bitcoin "Bitcoin Cash"
```

You can use the `-l` (or `--limit`) to limit the number of currencies fetched.
```
$ hodl-ticker -l 20
```

You can use the `-w` (or `--watch`) to trigger the watch mode, data will be fetch every 5 secs.
```
$ hodl-ticker -w
```

You can use the `-h` (or `--help`) to find all valid options of hodl-ticker


This project is highly inspired by coinmon: https://github.com/bichenkk/coinmon
