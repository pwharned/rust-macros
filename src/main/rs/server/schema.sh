pg_dump -d $DATABASE_URL -s | sed '/^--/d' >schema.sql
