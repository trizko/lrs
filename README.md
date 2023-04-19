# `lrs`

`lrs` is the port of `ls` to rust that nobody wanted.

![CI](https://github.com/trizko/lrs/actions/workflows/ci.yml/badge.svg?branch=main)

---

### Features
- Lists files and directories, just like `ls`, but without as many features.
- Multithreaded (not really but can be made that way, fearlessly).
- Guaranteed to provide at least 0.001% increased productivity to your workflow, or your money back (note: free software, no actual refunds).

### Installation
```sh
cargo install lrs
```

### Usage
Use `lrs` just like you'd use `ls`, and be prepared for a life-changing experience:

```sh
lrs
```

### FAQ

**Q: Why did you make this?**  
A: Because I am learning Rust and was bored.

**Q: How is this better than `ls`?**  
A: It's not. But it's written in Rust, so....

**Q: Can I contribute?**  
A: Sure, why not? Just submit a PR, if you're so inclined.

### Disclaimer

`lrs` is provided "as is", with no warranty, and no guarantee that it will change your life. It was created for fun and should be taken as such. If you're looking for serious file listing, stick to `ls`.