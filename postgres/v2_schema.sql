CREATE TABLE "eight".items (
    "id"                UUID            NOT NULL,
    "code"              INTEGER         NOT NULL,
    "name"              VARCHAR(255)    NOT NULL,
    "price"             INTEGER         NULL,
    "sell_price"        INTEGER         NULL,
    CONSTRAINT "item_pkey" PRIMARY KEY ("id")
);

CREATE TABLE "eight".item_inventories (
    "id"                UUID            NOT NULL,
    "item_id"           UUID            NULL,
    "quantity"          INTEGER         NULL,
    CONSTRAINT "item_inventory_pkey" PRIMARY KEY ("id"),
    CONSTRAINT "item_inventory_item_id_key" UNIQUE ("item_id")
);

CREATE TABLE "eight".weapons (
    "id"                UUID            NOT NULL,
    "name"              VARCHAR(255)    NOT NULL,
    "str"               INTEGER         NOT NULL,
    "hit"               INTEGER         NOT NULL,
    CONSTRAINT "weapon_pkey" PRIMARY KEY ("id")
);

CREATE TABLE "eight".weapon_remodels (
    "id"                UUID            NOT NULL,
    "price"             INTEGER         NULL,
    "weapon_id"         UUID            NULL,
    CONSTRAINT "weapon_remodel_pkey" PRIMARY KEY ("id"),
    CONSTRAINT "weapon_remodel_weapon_id_key" UNIQUE ("weapon_id")
);

CREATE TABLE "eight".weapon_materials (
    "id"                UUID            NOT NULL,
    "item_id"           UUID            NULL,
    "quantity"          INTEGER         NULL,
    "weapon_id"         UUID            NULL,
    CONSTRAINT "weapon_material_pkey" PRIMARY KEY ("id")
);

CREATE TABLE "eight"."weapon_remodel_materials" (
    "materials_id"      UUID            NOT NULL,
    "weapon_remodel_id" UUID            NOT NULL,
    CONSTRAINT "weapon_remodel_materials_materials_id_key" UNIQUE ("materials_id")
);

CREATE TABLE "eight".users (
    "id"            UUID NOT NULL,
    "username"      VARCHAR(255) NULL,
    "given_name"    VARCHAR(255) NULL,
    "family_name"   VARCHAR(255) NULL,
    "gender"        INTEGER NULL,
    "age"           INTEGER NULL,
    "profile"       VARCHAR(255) NULL,
    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
);

ALTER TABLE "eight".item_inventories ADD CONSTRAINT "fkis34mg9pc892kd0968g1rb9pt"
    FOREIGN KEY ("item_id") REFERENCES "eight".items ("id") ON DELETE NO ACTION ON UPDATE NO ACTION;
ALTER TABLE "eight".weapon_materials ADD CONSTRAINT "fkt2x7wtlcxltae6igo3eyujs71"
    FOREIGN KEY ("item_id") REFERENCES "eight".items ("id") ON DELETE NO ACTION ON UPDATE NO ACTION;
ALTER TABLE "eight".weapon_materials ADD CONSTRAINT "fkedkqwup8snwjpbrimdymyfbky"
    FOREIGN KEY ("weapon_id") REFERENCES "eight".weapons ("id") ON DELETE NO ACTION ON UPDATE NO ACTION;
ALTER TABLE "eight".weapon_remodels ADD CONSTRAINT "fk2m9w6s61ou2sc6pt25tq1n2pu"
    FOREIGN KEY ("weapon_id") REFERENCES "eight".weapons ("id") ON DELETE NO ACTION ON UPDATE NO ACTION;
ALTER TABLE "eight"."weapon_remodel_materials" ADD CONSTRAINT "fkw8eps8kb45v36wgo2glgqyc0"
    FOREIGN KEY ("materials_id") REFERENCES "eight".weapon_materials ("id") ON DELETE NO ACTION ON UPDATE NO ACTION;
ALTER TABLE "eight"."weapon_remodel_materials" ADD CONSTRAINT "fk1hcrwls25ugakyn57263vlllx"
    FOREIGN KEY ("weapon_remodel_id") REFERENCES "eight".weapon_remodels ("id") ON DELETE NO ACTION ON UPDATE NO ACTION;
