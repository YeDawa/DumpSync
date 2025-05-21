# DumpSync Command: Pull

The `pull` command allows you to download SQL files directly from a URL and import them into your database in one operation, without saving the file locally. This is particularly useful for quickly importing database dumps hosted on the internet or company servers.

## Usage

```bash
dumpsync pull <file_url> [options]
```

### Parameters

- `<file_url>`: The URL of the SQL file to download and import

## Examples

### Basic Usage

```bash
dumpsync pull https://example.com/database_dump.sql
```

This command will download the SQL file from the specified URL and directly import it into your configured database.
