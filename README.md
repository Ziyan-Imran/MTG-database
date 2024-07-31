# Overview

Create a database that contains MTG cards
Have an API that can search through the database and return a specific card depending on user-submitted CLI input

## Setting up a SQL Server Database

### For Ubuntu 20.04 Do the Following:

1. Import GPG Keys:

`wget -qO- https://packages.microsoft.com/keys/microsoft.asc | sudo apt-key add -`

2. Register the repo:

`sudo add-apt-repository "$(wget -qO- https://packages.microsoft.com/config/ubuntu/20.04/mssql-server-preview.list)"`

3. Update and Install SQL Server:

```bash
sudo apt-get update
sudo apt-get install -y mssql-server
```

4. Configure SQL server with mssql-conf:
   `sudo /opt/mssql/bin/mssql-conf setup`

5. Connect to SQL Server using 127.0.0.1 NOT LOCALHOST

### For Ubuntu 22.04

1. Download the Public key, convert from ASCII to GPG format, and write to the required location:

```
curl -fsSL https://packages.microsoft.com/keys/microsoft.asc | sudo gpg --dearmor -o /usr/share/keyrings/microsoft-prod.gpg

```

If you encounter a warning that the public key is not available, use the following command:

```
curl https://packages.microsoft.com/keys/microsoft.asc | sudo tee /etc/apt/trusted.gpg.d/microsoft.asc
```

2. Manually download and register the SQL Server Ubuntu repository:

```
curl -fsSL https://packages.microsoft.com/config/ubuntu/22.04/mssql-server-2022.list | sudo tee /etc/apt/sources.list.d/mssql-server-2022.list
```

3. Run the following commands to install SQL Server:

```
sudo apt-get update
sudo apt-get install -y mssql-server
```

4. Run `mssql-conf setup` and follow the prompts to set the SA password and choose your edition. The folloing SQL server editions are freely licensed: Evaluation, Developer, Express

```
sudo /opt/mssql/bin/mssql-conf setup
```

5. Once configuration is done, verify that the service is running:

```
systemctl status mssql-server --no-pager
```

6. If you plan to connect remotely, you might also need to open the SQL Server TCP port (default 1433) on your firewall.

## Setting up a PostgreSQL database

1. Ensure the following libraries are installed on the machine

```
sudo apt install postgresql postgresql-contrib
```

Can use the following three commands to confirm functionality

- `sudo service postgresql status` for checking the status of your database.
- `sudo service postgresql start` to start running your database.
- `sudo service postgresql stop` to stop running your database.

2. Run `psql mtg` to access the PostgreSQL database of the name MTG

- To make a database, use the following command `createdb <db_name>`
