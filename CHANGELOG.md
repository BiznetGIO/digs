# TBD

Unreleased changes. Release notes have not yet been written.

# 0.1.4 (2021-04-08)

digs 0.1.4 a minor version release that fixes config bugs.
Starting from this version the code-base also free from `unwrap`.
Thus hopefully ensure no `panic` will occur.

Bug fixes:

- [BUG #7](https://github.com/BiznetGIO/digs/pull/7): fix panic if default config also doesn't exist
- [BUG #8](https://github.com/BiznetGIO/digs/pull/8): avoid all panics with proper Error handling

# 0.1.3 (2021-04-01)

Feature enhancements:

- [FEATURE #5]: Return different exit code and give more context to the error

# 0.1.2 (2021-02-22)

Feature enhancements:

- [FEATURE #2]: Support querying CNAME record
- [FEATURE #2]: Use name server reply (SOA) if record answer is empty


# 0.1.1

Initial release
