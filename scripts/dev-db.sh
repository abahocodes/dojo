#!/bin/sh
# Rebuilds target/dev.db from migrations/ so sqlx's query macros can check SQL
# at compile time (DATABASE_URL in .env points at it).
#
#   scripts/dev-db.sh            rebuild the dev database
#   scripts/dev-db.sh --prepare  also refresh the offline cache in .sqlx/
#                                (commit it; CI and release builds use it)
set -eu
cd "$(dirname "$0")/.."

mkdir -p target
rm -f target/dev.db
for f in migrations/*.sql; do
    sqlite3 target/dev.db < "$f"
done
echo "built target/dev.db"

if [ "${1:-}" = "--prepare" ]; then
    rm -rf .sqlx
    mkdir .sqlx
    touch src/store/mod.rs
    SQLX_OFFLINE=false SQLX_OFFLINE_DIR="$PWD/.sqlx" cargo check --all-targets --quiet
    echo "wrote $(ls .sqlx | wc -l | tr -d ' ') queries to .sqlx/"
fi
