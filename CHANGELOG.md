# Changelog

All notable changes to this project will be documented in this file.

## [0.4.0] - 2024-02-25

### Features

- Validate address when reading the config ([02b741d](https://github.com/BiznetGIO/digs/commit/02b741dcded1107fcc895ac506b8df1a6e84bee7))
- Optional port in config file ([256c1a3](https://github.com/BiznetGIO/digs/commit/256c1a39d17c10c6f010743846aa099f9b700c42))
  - **BREAKING!** ⚠️ : optional port in config file

## [0.3.1] - 2023-01-04

### Performance

- Use `writeln!` for better performance ([80addf6](https://github.com/BiznetGIO/digs/commit/80addf646c8196accd2445451cc3dc4eb6b911cd))

## [0.3.0] - 2023-01-03

### Features

- Print config snippets of error location ([7879671](https://github.com/BiznetGIO/digs/commit/787967106ca614037cace3fc876f06aad0f5cd49))

## [0.2.2] - 2022-12-28

### Performance

- Migrate to `owo-color` ([64dc398](https://github.com/BiznetGIO/digs/commit/64dc398cf05da3920f5b9f8c5a7b0d082965d57e))

  It has fewer dependencies

## [0.2.1] - 2022-12-21

### Bug fixes

- Wrong crate name in error message ([5aa6f47](https://github.com/BiznetGIO/digs/commit/5aa6f47705afc2e45b2482b64a48579bc9dfeda5))

## [0.2.0] - 2022-12-21

### Features

- Better error messsage ([f069b3e](https://github.com/BiznetGIO/digs/commit/f069b3e423302ed04377f1662a5c603078e672c4))

### Bug fixes

- Change `--file` to `--config` ([4969414](https://github.com/BiznetGIO/digs/commit/49694145a80e0d5f8c10b7660a77822b93fdec93))

  It is more self-describable.

## [0.1.6] - 2021-04-20

### Bug fixes

- Tell user if no zone found ([4e5cd51](https://github.com/BiznetGIO/digs/commit/4e5cd51482ea69da364ff92101e5286e9e7a817d))

  If the target record is not found, show the default record (SOA).
  Otherwise, it's no zone.

## [0.1.5] - 2021-04-08

### Bug fixes

- Avoid all panics with proper error handling ([71a6e4b](https://github.com/BiznetGIO/digs/commit/71a6e4bbb33f75dac03579d897788e53d8a44e63))

  Make sure all the `unwrap()` is safe and has proper error handling

- Panic if default config also doesn't exist ([fb84831](https://github.com/BiznetGIO/digs/commit/fb84831eae98a09c21818f3678ad9ec644350ca0))

## [0.1.2] - 2021-02-22

### Features

- Use name server reply (SOA) if record answer is empty ([2e93189](https://github.com/BiznetGIO/digs/commit/2e93189efcd00e81280af6bd4d16b3fdda55a35e))

  If the answer is empty digs will display nothing. Instead of this
  behavior, digs will show the default reply of DNS name server which
  is a SOA record.

  Other DNS command-line client such as `dig` and `dog` also has this
  behavior.

- Support CNAME record ([4ce3318](https://github.com/BiznetGIO/digs/commit/4ce3318be855f2e0a9888fd7d5a0289afaf4cb20))

  Digs now able to query CNAME record
