DROP TABLE IF EXISTS WeaveVMArchiver;

CREATE TABLE IF NOT EXISTS WeaveVMArchiver (
    Id INT AUTO_INCREMENT PRIMARY KEY,
    NetworkBlockId INT UNIQUE,
    WeaveVMArchiveTxid VARCHAR(66) UNIQUE
);

CREATE TABLE IF NOT EXISTS WeaveVMArchiverBackfill (
    Id INT AUTO_INCREMENT PRIMARY KEY,
    NetworkBlockId INT UNIQUE,
    WeaveVMArchiveTxid VARCHAR(66) UNIQUE
);
