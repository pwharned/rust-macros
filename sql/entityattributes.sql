insert into entityattributes select a.asset_id, av.aid, av.id from assets a join attributevalues av on av.value = a.asset_geo where aid =6;
insert into entityattributes select a.asset_id, av.aid, av.id from assets a join brands b on a.asset_brand= b.brand_id join attributevalues av on av.value= b.brand_namei where av.aid = 1;
insert into entityattributes select a.asset_id, av.aid, av.id from assets a join practices p on practice_id = a.asset_practice join attributevalues av on av.value = p.practice_name where av.aid=2;
insert into entityattributes select a.asset_id, av.aid, av.id from assets a join asset_types at on at.type_id = a.asset_type join attributevalues av on av.value= at.type_name where av.aid = 4;
insert into entityattributes select a.asset_id, av.aid, av.id from assets a join offering_types at on at.offering_type_id = a.asset_offering_type join attributevalues av on av.value= at.offering_type_name where av.aid = 5;
insert into entityattributes select a.asset_id, av.aid, av.id from assets a join attributevalues av on av.value = a.asset_market where av.aid = 7;
insert into entityattributes select a.asset_id, av.aid, av.id from (select  asset_id, case when is_ip_cleared ='f' then 'False' else 'True' end as ipcleared from assets) a join attributevalues av on av.value = a.ipcleared where av.aid = 8;
insert into entityattributes select a.asset_id, av.aid, av.id from (select  asset_id, case when is_sellable ='f' then 'False' else 'True' end as sellable from assets) a join attributevalues av on av.value = a.sellable where av.aid = 9;

