DROP TABLE IF EXISTS WeaveVMArchiverRss3;
DROP TABLE IF EXISTS WeaveVMArchiverRss3Backfill;

CREATE TABLE IF NOT EXISTS WeaveVMArchiverRss3 (
    Id INT AUTO_INCREMENT PRIMARY KEY,
    NetworkBlockId INT UNIQUE,
    WeaveVMArchiveTxid VARCHAR(66) UNIQUE
);

CREATE TABLE IF NOT EXISTS WeaveVMArchiverRss3Backfill (
    Id INT AUTO_INCREMENT PRIMARY KEY,
    NetworkBlockId INT UNIQUE,
    WeaveVMArchiveTxid VARCHAR(66) UNIQUE
);

CREATE INDEX idx_archiver_txid ON WeaveVMArchiverRss3 (WeaveVMArchiveTxid);
CREATE INDEX idx_backfill_txid ON WeaveVMArchiverRss3Backfill (WeaveVMArchiveTxid);
CREATE INDEX idx_archiver_block_id ON WeaveVMArchiverRss3 (NetworkBlockId);
CREATE INDEX idx_backfill_block_id ON WeaveVMArchiverRss3Backfill (NetworkBlockId);
