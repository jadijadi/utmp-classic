# utmp-classic
Rust library for reading utmp files. Please note that all Unix like systems (Including all GNU/Linuxes, MacOS and all BSDs except OpenBSD) use the newer `utmpx` file format, even if they still call it `utmp`. This library works only for original Unix `utmp` files which is only used in OpenBSD as far as I know.

If you are looking for a lib to be used on anything other than OpenBSD; you might be looking for a `utmpx` library, although most of them calls themselves `utmp`; not sure why :D 

# sample run
A sample `utmp` file is included in the root directory, you can run a sample by issuing:

```
cargo run --package utmp-classic --example dump-utmp utmp 
```

# history
This library is based on `[utmp-rs](https://github.com/upsuper/utmp-rs)` library by *upsuper*; changed to work on the classic AT&T Unix v1 style `utmp` files still used by OpenBSD.