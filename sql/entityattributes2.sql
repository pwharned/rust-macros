INSERT INTO entityattributes
SELECT a.asset_id, av.aid, av.id
FROM assets a
JOIN attributevalues av ON av.value = a.asset_geo
WHERE av.aid = 6
  AND a.asset_geo IS NOT NULL
  AND NOT EXISTS (
      SELECT 1
      FROM entityattributes ea
      WHERE ea.eid = a.asset_id AND ea.aid = av.aid AND ea.vid = av.id
  );

INSERT INTO entityattributes
SELECT a.asset_id, av.aid, av.id
FROM assets a
JOIN brands b ON a.asset_brand = b.brand_id
JOIN attributevalues av ON av.value = b.brand_name
WHERE av.aid = 1
  AND a.asset_brand IS NOT NULL
  AND NOT EXISTS (
      SELECT 1
      FROM entityattributes ea
      WHERE ea.eid = a.asset_id AND ea.aid = av.aid AND ea.vid = av.id
  );

INSERT INTO entityattributes
SELECT a.asset_id, av.aid, av.id
FROM assets a
JOIN practices p ON p.practice_id = a.asset_practice
JOIN attributevalues av ON av.value = p.practice_name
WHERE av.aid = 2
  AND a.asset_practice IS NOT NULL
  AND NOT EXISTS (
      SELECT 1
      FROM entityattributes ea
      WHERE ea.eid = a.asset_id AND ea.aid = av.aid AND ea.vid = av.id
  );

INSERT INTO entityattributes
SELECT a.asset_id, av.aid, av.id
FROM assets a
JOIN asset_types at ON at.type_id = a.asset_type
JOIN attributevalues av ON av.value = at.type_name
WHERE av.aid = 4
  AND a.asset_type IS NOT NULL
  AND NOT EXISTS (
      SELECT 1
      FROM entityattributes ea
      WHERE ea.eid = a.asset_id AND ea.aid = av.aid AND ea.vid = av.id
  );

INSERT INTO entityattributes
SELECT a.asset_id, av.aid, av.id
FROM assets a
JOIN offering_types at ON at.offering_type_id = a.asset_offering_type
JOIN attributevalues av ON av.value = at.offering_type_name
WHERE av.aid = 5
  AND a.asset_offering_type IS NOT NULL
  AND NOT EXISTS (
      SELECT 1
      FROM entityattributes ea
      WHERE ea.eid = a.asset_id AND ea.aid = av.aid AND ea.vid = av.id
  );

INSERT INTO entityattributes
SELECT a.asset_id, av.aid, av.id
FROM assets a
JOIN attributevalues av ON av.value = a.asset_market
WHERE av.aid = 7
  AND a.asset_market IS NOT NULL
  AND NOT EXISTS (
      SELECT 1
      FROM entityattributes ea
      WHERE ea.eid = a.asset_id AND ea.aid = av.aid AND ea.vid = av.id
  );

INSERT INTO entityattributes
SELECT a.asset_id, av.aid, av.id
FROM (
    SELECT asset_id,
           CASE WHEN is_ip_cleared = 'f' THEN 'False' ELSE 'True' END AS ipcleared
    FROM assets
    where is_ip_cleared is not null
) a
JOIN attributevalues av ON av.value = a.ipcleared
WHERE av.aid = 8
  AND NOT EXISTS (
      SELECT 1
      FROM entityattributes ea
      WHERE ea.eid = a.asset_id AND ea.aid = av.aid AND ea.vid = av.id
  );

INSERT INTO entityattributes
SELECT a.asset_id, av.aid, av.id
FROM (
    SELECT asset_id,
           CASE WHEN is_sellable = 'f' THEN 'False' ELSE 'True' END AS sellable
    FROM assets
) a
JOIN attributevalues av ON av.value = a.sellable
WHERE av.aid = 9
  AND a.sellable IS NOT NULL
  AND NOT EXISTS (
      SELECT 1
      FROM entityattributes ea
      WHERE ea.eid = a.asset_id AND ea.aid = av.aid AND ea.vid = av.id
  );

