
# raspberrypi_iot_core_rust
    this is raspberrypi_iot_core_rust


<!--------------------------------------------------------------------------------- Source -->
<br><br>

## Source
```bash
git clone git@github.com:kashanimorteza/raspberrypi_iot_core_rust.git
cd raspberrypi_iot_core_rust
```



<!--------------------------------------------------------------------------------- Database -->
<br><br>

## Database
<!------------------------- Install -->
Install
```bash
sh -c 'echo "deb http://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list'
wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | sudo apt-key add -
apt update
apt -y install postgresql
```
<!------------------------- Config -->
Config
```bash
sed -i "s/#listen_addresses = 'localhost'/listen_addresses = '0.0.0.0'/" /etc/postgresql/17/main/postgresql.conf
sed -i "s/max_connections = 100/max_connections = 200/" /etc/postgresql/17/main/postgresql.conf
echo "host all all 0.0.0.0/0 md5" >> /etc/postgresql/17/main/pg_hba.conf
```
<!------------------------- Service -->
Service
```bash
sudo systemctl restart postgresql
sudo systemctl status postgresql
```
<!------------------------- Role -->
Role
```bash
sudo -u postgres psql
ALTER USER postgres WITH PASSWORD '123456';
CREATE ROLE raspberrypi WITH LOGIN PASSWORD '123456';
```
<!------------------------- Database -->
Database
```bash
sudo -u postgres psql
CREATE DATABASE raspberrypi;
DROP DATABASE raspberrypi;
```
```bash
PGPASSWORD='123456' psql -h 192.168.64.9 -U postgres -d postgres -c "CREATE DATABASE raspberrypi;"
PGPASSWORD='123456' psql -h 192.168.64.9 -U postgres -d postgres -c "DROP DATABASE raspberrypi;"
```
<!------------------------- Table -->
Database
```bash
PGPASSWORD='123456' psql -U postgres -h 192.168.64.9 -d raspberrypi -f db_postgres.sql
```