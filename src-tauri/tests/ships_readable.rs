//! The shipped database has to open from a directory the app cannot write.
//!
//! This is the one property of `content.db` that no amount of local testing
//! catches, because every directory a developer builds in is writable. It only
//! shows up after a machine-scope install, where a standard user has read and
//! execute on Program Files and nothing else.
//!
//! A WAL database creates a `-shm` file beside itself before it can be read.
//! That is true of a connection opened read-only, which is counterintuitive and
//! is exactly why it survived to a built installer once already. Sealing the
//! database into a rollback journal removes the requirement.

use compendium_lib::compile;
use compendium_lib::search::Index;
use std::path::{Path, PathBuf};

fn build(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("compendium-seal-{name}"));
    let content = dir.join("content");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&content).unwrap();

    std::fs::write(
        content.join("a-card.md"),
        "---\nid: a-card\ntitle: A card\ntype: section\ntrack: D\nvolatility: low\n\
         verified: 2026-08-02\nanswer: >\n  A card the compiler will accept.\n---\n\n\
         ## More\n\nSomething to find.\n\n## Full\n\nSomething to find, described \
         again at the length the linter expects of a full tier.\n",
    )
    .unwrap();

    let loaded = compile::load_cards(&content).unwrap();
    let db = dir.join("content.db");
    let mut conn = compile::create_database(&db).unwrap();
    compile::write_cards(&mut conn, &loaded).unwrap();
    compile::seal(&conn).unwrap();
    drop(conn);
    db
}

/// Header bytes 18 and 19 are the write and read format versions: 1 for a
/// rollback journal, 2 for WAL. Read from the file rather than asked of a
/// connection, because opening one is what creates the sidecar we are checking
/// for the absence of.
fn journal_bytes(db: &Path) -> (u8, u8) {
    let header = std::fs::read(db).unwrap();
    (header[18], header[19])
}

#[test]
fn a_sealed_database_is_not_in_wal_mode() {
    let db = build("mode");
    assert_eq!(
        journal_bytes(&db),
        (1, 1),
        "shipped in WAL mode, which needs a writable directory to read"
    );
}

#[test]
fn opening_a_sealed_database_writes_nothing_beside_it() {
    let db = build("sidecars");
    let dir = db.parent().unwrap();

    let index = Index::open(&db).unwrap();
    assert_eq!(index.card_count().unwrap(), 1);

    for suffix in ["-shm", "-wal"] {
        let sidecar = dir.join(format!("content.db{suffix}"));
        assert!(
            !sidecar.exists(),
            "opening the corpus created {}, so it cannot be read from Program Files",
            sidecar.display()
        );
    }
}
