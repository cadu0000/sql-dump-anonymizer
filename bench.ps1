param(
    [int]$Rows = 1000000,
    [string]$Mode = "insert"
)

# ==========================================
# CONFIG
# ==========================================
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path

function Find-ProjectRoot {
    $dir = $ScriptDir

    while ($dir -ne "") {
        if (Test-Path "$dir\Cargo.toml") {
            return $dir
        }
        $parent = Split-Path $dir -Parent
        if ($parent -eq $dir) { break }
        $dir = $parent
    }

    Write-Error "Cargo.toml not found"
    exit 1
}

$ProjectRoot = Find-ProjectRoot
$OutputDir = "$ProjectRoot\generated"

New-Item -ItemType Directory -Force -Path $OutputDir | Out-Null

$SQL_FILE = "$OutputDir\dump.sql.gz"
$TOML_FILE = "$OutputDir\rules.toml"
$OUTPUT_FILE = "$OutputDir\anonymized.sql.gz"
$STATS_FILE = "$OutputDir\stats.txt"

$BINARY = "$ProjectRoot\target\release\ghostdump.exe"

# ==========================================
# HEADER
# ==========================================
Write-Host "--------------------------------------"
Write-Host "   GhostDump Benchmark"
Write-Host "--------------------------------------"
Write-Host "Rows:        $Rows"
Write-Host "Mode:        $Mode"
Write-Host "--------------------------------------"

# ==========================================
# STEP 1: GENERATE DATA
# ==========================================
Write-Host "[1/4] Generating dataset..."
python "$ScriptDir\sql_generator.py" --rows $Rows --mode $Mode

# ==========================================
# STEP 2: BUILD
# ==========================================
Write-Host "[2/4] Building (release)..."
Set-Location $ProjectRoot
cargo build --release

# ==========================================
# STEP 3: RUN
# ==========================================
Write-Host "[3/4] Running GhostDump..."

$process = Start-Process -FilePath $BINARY `
    -ArgumentList "-c `"$TOML_FILE`" -i `"$SQL_FILE`" -o `"$OUTPUT_FILE`" -s benchmark-secret -p" `
    -NoNewWindow -PassThru

$start = Get-Date
$process.WaitForExit()
$end = Get-Date

# ==========================================
# STEP 4: METRICS
# ==========================================
Write-Host "[4/4] Calculating metrics..."

$elapsed = ($end - $start).TotalSeconds
$lines = Get-Content $STATS_FILE

$throughput = [int]($lines / [math]::Max($elapsed, 1))

$proc = Get-Process -Id $process.Id -ErrorAction SilentlyContinue

$memory = if ($proc) { [int]($proc.PeakWorkingSet64 / 1KB) } else { 0 }

# ==========================================
# OUTPUT
# ==========================================
Write-Host ""
Write-Host "=========== RESULTS ==========="
Write-Host "Elapsed time:        $([math]::Round($elapsed,2)) s"
Write-Host "Throughput:          $throughput rows/sec"
Write-Host "Rows processed:      $lines"
Write-Host "Max memory (RAM):    $memory KB"
Write-Host "=============================="
Write-Host ""