-- This is free and unencumbered software released into the public domain.

DROP TABLE IF EXISTS "rdf_config";

CREATE TABLE IF NOT EXISTS "rdf_config" (
    "key" text NOT NULL,
    "val" text NOT NULL,
    PRIMARY KEY ("key"),
    CHECK (length("key") > 0)
);
