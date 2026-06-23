# Changelog

All notable changes to this project will be documented in this file.

## [0.4.3](https://github.com/BiznetGIO/digs/compare/v0.4.2..v0.4.3) - 2026-06-23

### Documentation

- Better CHANGELOG format - ([3907d2b](https://github.com/BiznetGIO/digs/commit/3907d2bed75c320d76a269bdad05d21ca690ac33))

## [0.4.0](https://github.com/BiznetGIO/digs/compare/v0.3.1..v0.4.0) - 2024-02-25

### Features

- Validate address when reading the config - ([02b741d](https://github.com/BiznetGIO/digs/commit/02b741dcded1107fcc895ac506b8df1a6e84bee7))
- [**breaking**] Optional port in config file - ([256c1a3](https://github.com/BiznetGIO/digs/commit/256c1a39d17c10c6f010743846aa099f9b700c42))

### Documentation

- Update outdated documentation - ([e6a5285](https://github.com/BiznetGIO/digs/commit/e6a52859956a7e710fa8ea59fab541ea7570c714))
- Update CI badge - ([34306d1](https://github.com/BiznetGIO/digs/commit/34306d1e58c741b31d05d76fe236c08b8c955e40))
- Change license to MIT - ([ea4b503](https://github.com/BiznetGIO/digs/commit/ea4b503958e1ef2030aecea3f1335b57d9ba9124))
- Reformat - ([f21110a](https://github.com/BiznetGIO/digs/commit/f21110a445972bbd163404770044a8782d92c556))
- Add commit message guideline - ([9ac6190](https://github.com/BiznetGIO/digs/commit/9ac61904d2491187525c7115d473f05fc008101f))
- List more feature - ([8f1c98f](https://github.com/BiznetGIO/digs/commit/8f1c98f5868a72c6af5d1687eb42e97d7f623b0a))

## New Contributors ❤️

- @ made their first contribution in [#20](https://github.com/BiznetGIO/digs/pull/20)
- @lpmi-13 made their first contribution in [#19](https://github.com/BiznetGIO/digs/pull/19)

## [0.3.1](https://github.com/BiznetGIO/digs/compare/v0.3.0..v0.3.1) - 2023-01-04

### Performance

- Use `writeln!` for better performance - ([80addf6](https://github.com/BiznetGIO/digs/commit/80addf646c8196accd2445451cc3dc4eb6b911cd))

## [0.3.0](https://github.com/BiznetGIO/digs/compare/v0.2.2..v0.3.0) - 2023-01-03

### Features

- Print config snippets of error location - ([7879671](https://github.com/BiznetGIO/digs/commit/787967106ca614037cace3fc876f06aad0f5cd49))

## [0.2.2](https://github.com/BiznetGIO/digs/compare/v0.2.1..v0.2.2) - 2022-12-28

### Performance

- Migrate to `owo-color` - ([64dc398](https://github.com/BiznetGIO/digs/commit/64dc398cf05da3920f5b9f8c5a7b0d082965d57e))

## [0.2.1](https://github.com/BiznetGIO/digs/compare/v0.2.0..v0.2.1) - 2022-12-21

### Bug Fixes

- Wrong crate name in error message - ([5aa6f47](https://github.com/BiznetGIO/digs/commit/5aa6f47705afc2e45b2482b64a48579bc9dfeda5))

## [0.2.0](https://github.com/BiznetGIO/digs/compare/v0.1.7..v0.2.0) - 2022-12-21

### Features

- Better error messsage - ([f069b3e](https://github.com/BiznetGIO/digs/commit/f069b3e423302ed04377f1662a5c603078e672c4))

### Bug Fixes

- Change `--file` to `--config` - ([4969414](https://github.com/BiznetGIO/digs/commit/49694145a80e0d5f8c10b7660a77822b93fdec93))

## [0.1.6](https://github.com/BiznetGIO/digs/compare/v0.1.5..v0.1.6) - 2021-04-20

### Bug Fixes

- Tell user if no zone found - ([4e5cd51](https://github.com/BiznetGIO/digs/commit/4e5cd51482ea69da364ff92101e5286e9e7a817d))

### Documentation

- Add development docs - ([75e0d4d](https://github.com/BiznetGIO/digs/commit/75e0d4db1155c7da121d01a0e8c9a267cf8f3ecc))

## [0.1.5](https://github.com/BiznetGIO/digs/compare/v0.1.3..v0.1.5) - 2021-04-08

### Bug Fixes

- Avoid all panics with proper error handling - ([71a6e4b](https://github.com/BiznetGIO/digs/commit/71a6e4bbb33f75dac03579d897788e53d8a44e63))
- Panic if default config also doesn't exist - ([fb84831](https://github.com/BiznetGIO/digs/commit/fb84831eae98a09c21818f3678ad9ec644350ca0))

### Documentation

- Digs is now installable via cargo - ([9576641](https://github.com/BiznetGIO/digs/commit/957664115ac9e223b5a75ba9fe3d0991397aed8e))
- Add release checklist - ([2095bed](https://github.com/BiznetGIO/digs/commit/2095bed1dcca8bbab01df4f8e9b7f9e94fd1b43d))
- Fix license identifier - ([c2dc5d5](https://github.com/BiznetGIO/digs/commit/c2dc5d58822eca48a055f2ffec4910c638a58bf6))
- Add rationale - ([6bd2362](https://github.com/BiznetGIO/digs/commit/6bd2362ca80abb09608a87168431374a05463ae5))

## [0.1.2](https://github.com/BiznetGIO/digs/compare/v0.1.1..v0.1.2) - 2021-02-22

### Features

- Use name server reply (SOA) if record answer is empty - ([2e93189](https://github.com/BiznetGIO/digs/commit/2e93189efcd00e81280af6bd4d16b3fdda55a35e))
- Support CNAME record - ([4ce3318](https://github.com/BiznetGIO/digs/commit/4ce3318be855f2e0a9888fd7d5a0289afaf4cb20))

### Documentation

- Migrate under Biznet Gio Nusantara Organization - ([f792490](https://github.com/BiznetGIO/digs/commit/f792490206eabbbabc4e319724d798efca5320ee))

## [0.1.1] - 2021-02-19

### Documentation

- Update demo - ([87aa5a0](https://github.com/BiznetGIO/digs/commit/87aa5a05071732c2157bb75ddcefdd228a1f0895))

## New Contributors ❤️

- @azzamsa made their first contribution
