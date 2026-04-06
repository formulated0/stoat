# stoat

> (because stoats are tall like a stack (lmao)(lol)(rofl))

a rewrite of [tsoding's](https://www.youtube.com/@Tsoding) stack based language [porth](https://gitlab.com/tsoding/porth) in rust.

compiles into native assembly (x86-64)

## quickstart

(only linux support)

simulate program (mini interpreter)

```console
$ cargo run sim /examples/test.st
```

compile to asm and run natively

```console
$ cargo run com /examples/test.st && ./output
```

## todo

- [x] compiled
- [x] native
- [x] stack based
- [ ] turing complete
- [ ] statically typed
- [ ] self hosted (rust as bootstrap, then rewrite language in itself)
