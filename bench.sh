#!/usr/bin/env bash

set -euo pipefail

# ==========================================
# CONFIG
# ==========================================
ROWS=${1:-1000000}
MODE=${2:-insert}

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

find_project_root() {
  local dir="$SCRIPT_DIR"

  while [[ "$dir" != "/" ]]; do
    if [[ -f "$dir/Cargo.toml" ]]; then
      echo "$dir"
      return
    fi
    dir="$(dirname "$dir")"
  done

  echo "Error: Cargo.toml not found" >&2
  exit 1
}

PROJECT_ROOT="$(find_project_root)"
OUTPUT_DIR="$PROJECT_ROOT/generated"

mkdir -p "$OUTPUT_DIR"

SQL_FILE="$OUTPUT_DIR/dump.sql.gz"
TOML_FILE="$OUTPUT_DIR/rules.toml"
OUTPUT_FILE="$OUTPUT_DIR/anonymized.sql.gz"
STATS_FILE="$OUTPUT_DIR/stats.txt"

BINARY="$PROJECT_ROOT/target/release/ghostdump"

# detect time flavor
TIME_CMD="/usr/bin/time"
TIME_ARGS="-v"

if ! $TIME_CMD -v true 2>/dev/null; then
  TIME_ARGS="-l" # macOS fallback
fi

# ==========================================
# HEADER
# ==========================================
echo "--------------------------------------"
echo "   GhostDump Benchmark"
echo "--------------------------------------"
echo "Rows:        $ROWS"
echo "Mode:        $MODE"
echo "--------------------------------------"

# ==========================================
# STEP 1: GENERATE DATA
# ==========================================
echo "[1/4] Generating dataset..."
python3 "$SCRIPT_DIR/sql_generator.py" --rows "$ROWS" --mode "$MODE"

# ==========================================
# STEP 2: BUILD
# ==========================================
echo "[2/4] Building (release)..."
cd "$PROJECT_ROOT"
cargo build --release

# ==========================================
# STEP 3: RUN
# ==========================================
echo "[3/4] Running GhostDump..."

START_TIME=$(date +%s)

$TIME_CMD $TIME_ARGS \
"$BINARY" \
  -c "$TOML_FILE" \
  -i "$SQL_FILE" \
  -o "$OUTPUT_FILE" \
  -s "benchmark-secret" \
  -p \
  > "$OUTPUT_DIR/run.log" 2> "$OUTPUT_DIR/time.log"

END_TIME=$(date +%s)

# ==========================================
# STEP 4: METRICS
# ==========================================
echo "[4/4] Calculating metrics..."

ELAPSED=$((END_TIME - START_TIME))
LINES=$(cat "$STATS_FILE")

THROUGHPUT=$((LINES / (ELAPSED > 0 ? ELAPSED : 1)))

# Linux (time -v)
MAX_MEM=$(grep -i "Maximum resident set size" "$OUTPUT_DIR/time.log" | awk '{print $6}')
USER_TIME=$(grep -i "User time" "$OUTPUT_DIR/time.log" | awk '{print $4}')
SYS_TIME=$(grep -i "System time" "$OUTPUT_DIR/time.log" | awk '{print $4}')
PAGE_FAULTS=$(grep -i "Major" "$OUTPUT_DIR/time.log" | awk '{print $6}')

# macOS fallback
if [[ -z "$MAX_MEM" ]]; then
  MAX_MEM=$(grep "maximum resident set size" "$OUTPUT_DIR/time.log" | awk '{print $1}')
  USER_TIME=$(grep "user" "$OUTPUT_DIR/time.log" | awk '{print $1}')
  SYS_TIME=$(grep "sys" "$OUTPUT_DIR/time.log" | awk '{print $1}')
  PAGE_FAULTS=$(grep "page faults" "$OUTPUT_DIR/time.log" | awk '{print $1}')
fi

# ==========================================
# OUTPUT
# ==========================================
echo ""
echo "=========== RESULTS ==========="
echo "Elapsed time:        ${ELAPSED}s"
echo "User time:           ${USER_TIME}s"
echo "System time:         ${SYS_TIME}s"
echo "Throughput:          ${THROUGHPUT} rows/sec"
echo "Rows processed:      ${LINES}"
echo "Max memory (RAM):    ${MAX_MEM} KB"
echo "Page faults (I/O):   ${PAGE_FAULTS}"
echo "=============================="
echo ""
echo "Logs:"
echo "  -> $OUTPUT_DIR/time.log"