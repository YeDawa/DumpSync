<div align='center'><img src='https://i.imgur.com/N7mW943.png'/></div>

A simple tool for dump and restore a MySQL database. It can be used for backup and restore purposes, or for transferring a database from one server to another.

## Overview

This tool allows for automatic dumps of a MySQL database at configurable time intervals, saving the files in a specified directory. Configuration can be done via environment variables or command-line arguments, providing flexibility for different usage contexts.

To Install using [crates.io](https://crates.io):

```bash
cargo install dumpsync
```

### Usage Example

**To make a dump, use like this:**:

```bash
dumpsync export -d my_database -i 3600 -f /path/to/
```

**To restore a dump, use like this**:

```bash
dumpsync import -f /path/to/dump.sql
```

**Environment Variables**:

```.env
DB_HOST="YOUR_HOST"
DB_PORT="YOUR_PORT"
DB_USER="YOUR_USERNAME"
DB_PASSWORD="YOUR_PASSWORD"
DB_NAME="YOUR_DATABASE"

DS_DUMP_INTERVAL="3600"
DS_DUMP_PATH="/path/to/"

# OR

DS_DB_HOST="YOUR_HOST"
DS_DB_PORT="YOUR_PORT"
DS_DB_USER="YOUR_USERNAME"
DS_DB_PASSWORD="YOUR_PASSWORD"
DS_DB_NAME="YOUR_DATABASE"

DS_DUMP_INTERVAL="3600"
DS_DUMP_PATH="/path/to/"
```

**Optional settings file**:

```yaml
exports:
  dump_data: true
  drop_table_if_exists: true
  database_if_not_exists: true
```

Save as `dumpsync.yaml` to the same directory where your project is running.

### Notes

- The tool requires read and write permissions in the directory set for `backup_path`.
- The MySQL database must be accessible for the dump to proceed.
- It is recommended to test the initial configuration to ensure that parameters are correct and the backup executes as expected.

### What's New

- Added support to new environment variables for connection.
- Added support to `DROP TABLE IF EXISTS` in the dump file.
- Added support to `CREATE DATABASE IF NOT EXISTS` in the dump file.
- Added support to `EXPORT` command to make a dump file.
- Added support to `IMPORT` command to restore a dump file.
- Added support to custom settings file.
