CREATE TABLE attributes (
        ID INT PRIMARY key GENERATED ALWAYS AS IDENTITY,
        NAME VARCHAR(255) NOT NULL
);

CREATE TABLE attributevalues (
      ID INT PRIMARY key GENERATED ALWAYS AS IDENTITY,
      AID INT NOT NULL,
      VALUE VARCHAR(255) NOT NULL
);

ALTER TABLE attributevalues
ADD CONSTRAINT attributevalues_attributes_fk 
FOREIGN KEY (aid) REFERENCES attributes(id);

CREATE TABLE entityattributes (
        EID UUID NOT NULL,
        AID INT NOT NULL,
        VID INT NOT NULL,
        PRIMARY KEY (EID, AID, VID)
);

ALTER TABLE entityattributes
ADD CONSTRAINT entityattributes_attributes_fk 
FOREIGN KEY (aid) REFERENCES "attributes"(id);

ALTER TABLE entityattributes 
ADD CONSTRAINT entityattributes_attributevalues_fk 
FOREIGN KEY (vid) REFERENCES attributevalues(id);

ALTER TABLE entityattributes 
ADD CONSTRAINT entityattributes_entities_fk 
FOREIGN KEY (eid) REFERENCES assets(asset_id);

create table parent ( 
  id int primary key GENERATED ALWAYS AS IDENTITY,
  PAID int not null,
  CAID int not null
);

ALTER TABLE parent 
ADD CONSTRAINT parent_attributes_fk 
FOREIGN KEY (paid) REFERENCES "attributes"(id);

ALTER TABLE parent 
ADD CONSTRAINT parent_attributes_fk_1 
FOREIGN KEY (caid) REFERENCES "attributes"(id);

create table relationship (
  id int primary key GENERATED ALWAYS AS IDENTITY,
  PAID int not null,
  CAID int not null,
  PAVID int not null,
  CAVID int not null
);

ALTER TABLE relationship 
ADD CONSTRAINT relationship_attributes_fk 
FOREIGN KEY (paid) REFERENCES "attributes"(id);

ALTER table relationship 
ADD CONSTRAINT relationship_attributes_fk_1 
FOREIGN KEY (caid) REFERENCES "attributes"(id);

ALTER TABLE relationship 
ADD CONSTRAINT relationship_attributevalues_fk 
FOREIGN KEY (pavid) REFERENCES attributevalues(id);

ALTER TABLE relationship 
ADD CONSTRAINT relationship_attributevalues_fk_1 
FOREIGN KEY (cavid) REFERENCES attributevalues(id);
