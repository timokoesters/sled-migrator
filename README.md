# Sled Migrator

Usage:
```
cargo run --release
```

The program will ask for the input path (path of your current sled db), the
output path (path where you want the migrated sled db to be) and the target
sled version (one of [0.31, 0.32, 0.33, 0.34]) and will upgrade the db to that
version.
