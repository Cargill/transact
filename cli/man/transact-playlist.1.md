% TRANSACT-PLAYLIST(1) Cargill, Incorporated | Transact Commands
<!--
  Copyright 2018-2021 Cargill Incorporated
  Licensed under Creative Commons Attribution 4.0 International License
  https://creativecommons.org/licenses/by/4.0/
-->

NAME% TRANSACT-WORKLOAD(1) Cargill, Incorporated | Transact Commands
<!--
  Copyright 2018-2021 Cargill Incorporated
  Licensed under Creative Commons Attribution 4.0 International License
  https://creativecommons.org/licenses/by/4.0/
-->

NAME
====

**transact-workload** — Submits a workload against a distributed ledger

SYNOPSIS
========
**transact workload** \[**FLAGS**\] \[**SUBCOMMAND**\]

DESCRIPTION
===========
This command is intended to be used in performance and stability testing
of distributed ledgers by continuously submitting batches to the provided
target groups at some rate.

The workload tool assumes the distributed ledger's REST API supports Cylinder
JWT authentication.

FLAGS
=====
`-h`, `--help`
: Prints help information

`-q`, `--quiet`
: Decrease verbosity (the opposite of -v). When specified, only errors or
  warnings will be output.

`-V`, `--version`
: Prints version information

`-v`
: Increases verbosity (the opposite of -q). Specify multiple times for more
  output.

OPTIONS
=======
`-k, --key PRIVATE-KEY-FILE`
: Specifies the full path to the private key file. The key will be used to
  sign the batches as well as generate a JWT for authentication.

`--smallbank-num-accounts ACCOUNTS`
: The number of smallbank accounts to make. (default: `100`)

`--smallbank-seed SEEDS`
: An integer to use as a seed to make the smallbank workload reproducible.

`--target-rate TARGET-RATE`
: How many batches to submit per second, either provide a number or a range
  with the min and max separated by `-` ex: `5-15`. (default: `1`)

`--targets TARGETS`
:  Node URLS to submit batches to, combine groups with `;`. Each group will get
   batches from a different workload generator to handle dependency
   requirements. The URL should include all of the information required to
   append `/batches` to the end.

`-u, --update UPDATE `
: The time in seconds between updates. The workloads will log the success rate
  of submitting the HTTP requests. (default: `30`)

`--workload WORKLOAD `
: The workload to be submitted. The possibly value is `smallbank`. The smallbank
  workload submits batches containing sabre Transactions for smallbank payloads.

EXAMPLES
========
The following shows starting 1 workload against a Splinter circuit `jEWSK-jdjSM`
with scabbard services. A Scabbard service runs a sabre transaction handler.
The smallbank smart contract must already be submitted to scabbard.

```
transact workload \
  --target-rate 1 \
  --key ./alice.priv \
  --workload smallbank \
  --update 5 \
  --smallbank-seed 10 \
  --smallbank-num-accounts 5 \
  --targets "http://0.0.0.0:8089/scabbard/jEWSK-jdjSM/a001;http://0.0.0.0:8088/scabbard/jEWSK-jdjSM/
```


SEE ALSO
========
| `transact(1)`
|
| Transact documentation: https://docs.rs/transact/latest

====

**transact-playlist** — Create and process playlists of pregenerated payloads

SYNOPSIS
========
**transact playlist** \[**FLAGS**\] \[**SUBCOMMAND**\]

DESCRIPTION
===========
This command can be used to generate files of pregenerated payloads,
transactions, and batches. The file containing the batches can then be submitted
against a distributed ledger.

Payload, transactions and batch generation can be very expensive and can skew
performance results during testing. Using a pregenerated batch file makes for a
more accurate and repeatable test.

FLAGS
=====
`-h`, `--help`
: Prints help information

`-q`, `--quiet`
: Decrease verbosity (the opposite of -v). When specified, only errors or
  warnings will be output.

`-V`, `--version`
: Prints version information

`-v`
: Increases verbosity (the opposite of -q). Specify multiple times for more
  output.

SUBCOMMANDS
===========
`batch`
: Generates signed batches from transaction input. The transaction input is
  expected to be length-delimited protobuf Transaction messages, which should
  also be pre-signed for submission to the validator.

`create`
: Generates a workload transaction playlist. A playlist is a series of
  transactions, described in YAML. This command generates a playlist and writes
  it to file or standard out.

`help`
:  Prints this message or the help of the given subcommand(s)

`process`
: Processes a transaction playlist. A playlist is a series of transactions,
  described in YAML.  This command processes a playlist, converting it into
  transactions and writes it to file or standard out.

`submit`
: Submits signed batches to one or more targets from batch input. The batch
  input is expected to be length-delimited protobuf Batch messages, which
  should also be pre-signed for submission to the validator.

SEE ALSO
========
| `transact(1)`
| `transact-playlist-batch(1)`
| `transact-playlist-create(1)`
| `transact-playlist-process(1)`
| `transact-playlist-submit(1)`
|
| Transact documentation: https://docs.rs/transact/latest
