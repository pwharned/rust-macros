psql $DATABASE_URL -c "create table geos(parent text, child text);"
psql $DATABASE_URL -c "\copy  geos from 'geo.csv' with (format csv, delimiter ',');"
psql $DATABASE_URL -c "insert into relationship(paid,pavid,caid,cavid) select av.aid as paid, av.id as pavid, av2.aid as caid, av2.id as cavid from geos g join attributevalues av on g.parent = av.value join attributevalues av2 on g.child = av2.value;"


