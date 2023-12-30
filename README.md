# rusty-tunes
Navigating through old iTunes xml library with the help of Rust

## building

```console
$ cargo build --release
```

## basic use

```
Usage: rusty-tunes <MODE> <PATH>
```

* list all deezer matched tracks (console mode)
```console
$ ./target/release/rusty-tunes T mylibrary.xml
```

* list all playlists (console mode)
```console
$ ./target/release/rusty-tunes P mylibrary.xml
```

* open random tracks (GUI mode)
```console
$ ./target/release/rusty-tunes R mylibrary.xml
```
