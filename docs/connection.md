# Database Connection

To connect to the database, you need to configure the Environment System in your `.env` file. You should include the following settings:

```dotenv
DB_HOST="YOUR_HOST"
DB_PORT="YOUR_PORT"
DB_USER="YOUR_USERNAME"
DB_PASSWORD="YOUR_PASSWORD"
DB_NAME="YOUR_DATABASE"

DS_DUMP_INTERVAL="3600"  # Interval for the dump process in seconds
DS_DUMP_PATH="/path/to/"  # Path where dumps will be saved

# OR, alternatively

DS_DB_HOST="YOUR_HOST"
DS_DB_PORT="YOUR_PORT"
DS_DB_USER="YOUR_USERNAME"
DS_DB_PASSWORD="YOUR_PASSWORD"
DS_DB_NAME="YOUR_DATABASE"

DS_DUMP_INTERVAL="3600"  # Interval for the dump process in seconds
DS_DUMP_PATH="/path/to/"  # Path where dumps will be saved
```

### Configuration Details

- **DB_HOST** / **DS_DB_HOST**: The hostname or IP address of your database server.
- **DB_PORT** / **DS_DB_PORT**: The port number on which your database is listening.
- **DB_USER** / **DS_DB_USER**: Your database username.
- **DB_PASSWORD** / **DS_DB_PASSWORD**: The password associated with the database user.
- **DB_NAME** / **DS_DB_NAME**: The name of the database you want to connect to.
- **DS_DUMP_INTERVAL**: The interval (in seconds) for the dump process; defaults to 3600 seconds (1 hour).
- **DS_DUMP_PATH**: The directory path where the database dumps will be saved.

You can choose to use either the `DB_` prefixed variables or the `DS_` prefixed variables for your configuration. Make sure to adjust the values accordingly to fit your environment.
