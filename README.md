# Sled Migrator

Usage:
```
cargo run --release
```

The program will ask for the input path (path of your current sled db), the
output path (path where you want the migrated sled db to be) and the target
sled version (one of [0.31, 0.32, 0.33, 0.34]) and will upgrade the db to that
version.

Or you can provide the values directly as parameters
```bash
cargo run --release -- --input=${PATH_TO_DB} --output=${OUTPUT_PATH} --target=0.34
```

### Docker

```bash
docker run --rm --volume ${NAME_OF_YOUR_CONDUIT_DATA_VOLUME}:/srv/conduit/.local/share/conduit matrixconduit/sled-migrator:latest /srv/conduit/sled-migrator --input=/srv/conduit/.local/share/conduit/${DOMAIN_NAME} --output=/srv/conduit/.local/share/conduit/${DOMAIN_NAME} --target=0.34
```
