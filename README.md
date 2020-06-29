# Quoter

![build-status](https://github.com/wrobbins/quoter/workflows/Build/badge.svg)

A simple CLI app in Rust to obtain stock quotes.

This is an exercise to get exposure to [Rust](https://www.rust-lang.org/).

![Example](./docs/example.svg)

## TODO

- [x] replace reqwest with [ureq](https://github.com/algesten/ureq)
- [x] accept 1-N sybols on the cli
- [ ] config file for saving default symbols
- [x] pretty output options:
  - [colored](https://github.com/mackwic/colored)
  - ~~[prettytable](https://github.com/phsym/prettytable-rs)~~ not needed
- [ ] optional configs for calculating cost basis

### Future

- [ ] support for alternative APIs
  - IE [this](https://financialmodelingprep.com/developer/docs/)
- [ ] fancy unicode icons for :arrow_up: :arrow_down:
- [ ] publish on `brew` `apt` `crates`

### Research

Similar projects

- [stock snippet](https://github.com/alexanderepstein/Bash-Snippets/tree/master/stocks)
- [Ticker.sh](https://github.com/pstadler/ticker.sh)