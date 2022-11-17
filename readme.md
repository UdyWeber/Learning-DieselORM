# Before Running the application make Sure that Postgress is Running
mac -> brew services start postgresql

# Diesel Commands and Whys
diesel migration run -> Run all created migrations
diesel migration redo -> Update Existent Migrations
diesel database reset -> Wipe all records from DB

# DB interaction
psql postgres://localhost/db -> Opens DB shell so you can execute SQL