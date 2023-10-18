# Vault

Vault is a key value store for your keys. The keys are stored and persisted in json format file.

* **Install**

```
cargo install --git https://github.com/jcbritobr/vault
```
* **How to use?**

```
$ vault -h
A key value tool to persist your passwords

Usage: vault <COMMAND>

Commands:
  read    Read data from storage
  create  Create an empty storage
  insert  Insert an item in storage
  remove  Remove an item from storage
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

* **Create storage**
```
$ vault create ~/.storage
```

* **Insert**

```
$ vault insert ~/.storage "user" "key" "description"
```

* **List**
```
$ vault read ~/Documents/vstorage2 -l
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
$ vault remove ~/.storage 1
```