# Database Connection

To connect to the database, you need to configure the Environment System in your `.env` file. You should include the following settings:

```dotenv
DB_HOST="YOUR_HOST" # Database host
DB_PORT="YOUR_PORT" # Database port
DB_USER="YOUR_USERNAME" # Database username
DB_PASSWORD="YOUR_PASSWORD" # Database password
DB_NAME="YOUR_DATABASE" # Database name

DS_DUMP_INTERVAL="3600"  # Interval for the dump process in seconds
DS_DUMP_PATH="/path/to/"  # Path where dumps will be saved

DS_TRANSFER_HOST="YOUR_TRANSFER_HOST" # Transfer database host
DS_TRANSFER_PORT="YOUR_TRANSFER_PORT" # Transfer database port
DS_TRANSFER_USER="YOUR_TRANSFER_USERNAME" # Transfer database username
DS_TRANSFER_PASSWORD="YOUR_TRANSFER_PASSWORD" # Transfer database password
DS_TRANSFER_DB_NAME="YOUR_TRANSFER_DATABASE" # Transfer database name

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
- **DS_TRANSFER_HOST**: The hostname or IP address of the transfer database server.
- **DS_TRANSFER_PORT**: The port number on which the transfer database is listening.
- **DS_TRANSFER_USER**: Your transfer database username.
- **DS_TRANSFER_PASSWORD**: The password associated with the transfer database user.
- **DS_TRANSFER_DB_NAME**: The name of the transfer database you want to connect to.

You can choose to use either the `DB_` prefixed variables or the `DS_` prefixed variables for your configuration. Make sure to adjust the values accordingly to fit your environment.
