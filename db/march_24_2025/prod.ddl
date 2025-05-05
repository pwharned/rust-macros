begin;
alter table assets alter column asset_brand drop not null;
alter table assets alter column asset_practice drop not null;
alter table assets alter column asset_geo drop not null;
alter table assets alter column asset_market  drop not null;
end;
