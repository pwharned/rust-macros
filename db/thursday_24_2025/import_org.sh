# get the distinct org values
#
psql $DATABASE_URL --csv -c "select distinct asset_owner from assets;" > emails;
while IFS= read -r line; do ./find_org.sh $line >> orgs; done < emails;
cat orgs | awk -F';' '{print $2}' | sort | uniq | grep . > distinct_orgs
psql $DATABASE_URL -c "create table avalues(value text);"
psql $DATABASE_URL -c "\copy avalues(value) from 'distinct_orgs' with(format csv, delimiter ';');"
psql $DATABASE_URL -c "insert into attributevalues(aid,value) select (select id from attributes where name = 'Org'), av.value from avalues av where av.value not in (select value from attributevalues); "
psql $DATABASE_URL -c "drop table avalues;"
psql $DATABASE_URL -c "create table email_mappings(email text, org text);" 
psql $DATABASE_URL -c "\copy email_mappings(email, org) from 'orgs' with(format csv, delimiter ';')"
psql $DATABASE_URL -c "insert into entityattributes (eid,vid,aid) select distinct a.asset_id as eid, av.id as vid, av.aid from assets a join email_mappings e on e.email = a.asset_owner join attributevalues av on av.value=e.org where cast(a.asset_id as text) ||av.id ||av.aid not in (select cast(eid as text)||vid||aid from entityattributes);"
psql $DATABASE_URL -c "drop table email_mappings;"
