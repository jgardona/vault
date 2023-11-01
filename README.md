# SYK

Syk (store your keys) is a key value store for your keys. The keys are stored and persisted in json format file.

* **Install**

```
cargo install syk
```
* **How to use?**

```
$ syk -h
A key value tool to persist your passwords

Usage: syk <COMMAND>

Commands:
  read    Read data from storage
  create  Create an empty storage
  insert  Insert an item in storage
  remove  Remove an item from storage
  lock    Encrypt and compress storage data
  unlock  Decrypt and decompress storage data
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

* **Create storage**
```
$ syk create ~/.storage
```

* **Insert**

```
$ syk insert ~/.storage "user" "key" "description"
```

* **List**
```
$ syk read ~/.storage -l
+----+---------------------+-----------------+-----------------------------+
| id | user                | password        | description                 |
+----+---------------------+-----------------+-----------------------------+
| 3  | test@gmail.com      | zxcffddxxssdddd | bbbbbbbbbbb                 |
+----+---------------------+-----------------+-----------------------------+
| 2  | test@gmail.com      | zxcffddxxssdddd | ccccccccccccccccccccccccccc |
+----+---------------------+-----------------+-----------------------------+
| 1  | test@gmail.com      | zxcffddxxssdddd | kkkkkkkkkkkkkkkkkk          |
+----+---------------------+-----------------+-----------------------------+
```

* **Remove**

```
$ syk remove ~/.storage 1
```

* **Lock**

Lock will encrypt and compress your storage to make it safe.

```
$ syk lock ~/.storage ~/.storage_package
```

* **Unlock**

Unlock will decompress and decrypt your storage to make it readable.

```
$ syk unlock ~/.storage_package ~/.storage
```
