Postgresql
reference: https://www.sqlshack.com/setting-up-a-postgresql-database-on-mac/

# macOS

```
  brew install postgresql

  # start
  brew services start postgresql
  # stop
  brew services stop postgresql

  # open postgres interface
  psql postgres
  # create role 'postgres'
  CREATE ROLE postgres WITH LOGIN PASSWORD ‘password’;
  ALTER ROLE postgres CREATEDB;

  # quit
  \q
  # open postgres by 'postgres' role
   psql postgres -U postgres


  $ cargo install diesel_cli --no-default-features --features postgres
  $ diesel setup
  $ diesel migration generate create_boards
```