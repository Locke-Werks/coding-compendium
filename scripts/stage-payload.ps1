<#
.SYNOPSIS
Assembles installer/payload from the release build.

.DESCRIPTION
Three things go in: the signed executable, the compiled corpus, and the
embedding weights. Everything the installer puts on disk comes from here, so
what this script excludes matters as much as what it copies.

Run it after `pnpm tauri build` and after signing the executable, then forge.

.PARAMETER AllowUnsigned
Stage an unsigned executable anyway. Payload members are extracted verbatim, so
an unsigned binary going in is an unsigned binary on the user's disk: SmartScreen on
first run and no publisher on the file. For local testing only.
#>
[CmdletBinding()]
param(
    [switch] $AllowUnsigned
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$repo    = Split-Path $PSScriptRoot -Parent
$payload = Join-Path $repo 'installer\payload'
$exe     = Join-Path $repo 'src-tauri\target\release\coding-compendium.exe'
$db      = Join-Path $repo 'build\content.db'
$cache   = Join-Path $repo '.fastembed_cache'

function Require-Path([string] $Path, [string] $How) {
    if (-not (Test-Path $Path)) {
        throw "missing: $Path`n  $How"
    }
}

Require-Path $exe   'build it with: pnpm tauri build'
Require-Path $db    'build it with: pnpm build:content'
Require-Path $cache 'populate it with: pnpm build:content (it downloads the weights once)'

# A binary built without tauri's custom-protocol feature serves the frontend from
# the dev server instead of from its own resources, so an installed copy opens to
# "localhost refused to connect". It is optimized, stripped and signable, and
# indistinguishable from a working build until someone runs it.
#
# The check is for the assets themselves, not for the dev URL: tauri serialises
# the whole config into the image either way, so the dev URL is present in both
# builds and proves nothing. The hashed asset names appear only when the frontend
# was actually compiled in. That also catches a frontend rebuilt after the binary,
# because the hash in dist\index.html would no longer be the one embedded.
$indexHtml = Join-Path $repo 'dist\index.html'
Require-Path $indexHtml 'build it with: pnpm build'

$assets = ([regex]'(?:src|href)="[^"]*/(assets/[^"]+)"').Matches(
    (Get-Content $indexHtml -Raw)) | ForEach-Object { Split-Path $_.Groups[1].Value -Leaf }
if (-not $assets) { throw "no hashed assets referenced by $indexHtml" }

$image = [IO.File]::ReadAllBytes($exe)
foreach ($asset in $assets) {
    $needle = [Text.Encoding]::UTF8.GetBytes($asset)
    $found  = $false
    for ($i = 0; $i -le $image.Length - $needle.Length -and -not $found; $i++) {
        if ($image[$i] -eq $needle[0]) {
            $found = $true
            for ($j = 1; $j -lt $needle.Length; $j++) {
                if ($image[$i + $j] -ne $needle[$j]) { $found = $false; break }
            }
        }
    }
    if (-not $found) {
        throw "coding-compendium.exe does not carry $asset, so it would load the " +
              "frontend from the dev server and open to a connection error. Rebuild " +
              "with: cargo build --release --features custom-protocol"
    }
}
Write-Host ("frontend embedded: {0}" -f ($assets -join ', '))

# An unsigned payload binary is the failure this script exists to prevent. It is
# invisible until someone else runs the installer, which is far too late.
$sig = Get-AuthenticodeSignature $exe
if ($sig.Status -ne 'Valid') {
    $msg = "coding-compendium.exe is $($sig.Status). Sign it before staging: payload " +
           "members extract verbatim, so it stays unsigned on disk."
    if (-not $AllowUnsigned) { throw $msg }
    Write-Warning $msg
}

if (Test-Path $payload) { Remove-Item $payload -Recurse -Force }
New-Item -ItemType Directory $payload | Out-Null

Copy-Item $exe $payload
Copy-Item $db  $payload

# content.db ships alone. SQLite writes -shm and -wal beside a database it has
# opened, and staging those means shipping a stale journal next to a read-only
# file in Program Files, where the recovery it asks for cannot happen.
Get-ChildItem $payload -Filter 'content.db-*' | Remove-Item -Force

# The weights, flattened.
#
# fastembed resolves a file by reading refs/<revision> for a commit hash and
# then looking for snapshots/<hash>/<name>. It never reads blobs/. In a real
# HuggingFace cache the snapshot entries are symlinks into blobs/, which would
# cost us the 66 MB model twice: once as the blob and once as the resolved
# symlink the packager follows. Copying the content into the snapshot and
# dropping blobs/ leaves the layout fastembed actually reads.
$modelSrc = Get-ChildItem $cache -Directory | Where-Object Name -like 'models--*'
if (-not $modelSrc) { throw "no model directory under $cache" }

foreach ($model in $modelSrc) {
    $dst = Join-Path $payload ".fastembed_cache\$($model.Name)"

    $refs = Join-Path $model.FullName 'refs'
    Copy-Item $refs (Join-Path $dst 'refs') -Recurse -Force

    foreach ($snapshot in Get-ChildItem (Join-Path $model.FullName 'snapshots') -Directory) {
        $out = Join-Path $dst "snapshots\$($snapshot.Name)"
        New-Item -ItemType Directory $out -Force | Out-Null
        foreach ($file in Get-ChildItem $snapshot.FullName -File) {
            # Resolve through the reparse point rather than copying the link.
            $bytes = [System.IO.File]::ReadAllBytes($file.FullName)
            [System.IO.File]::WriteAllBytes((Join-Path $out $file.Name), $bytes)
        }
    }
}

# Lock files are zero bytes, and an empty member is a stored-size-zero record the
# container reader treats as malformed on older stubs. They carry nothing.
Get-ChildItem $payload -Recurse -File -Filter '*.lock' | Remove-Item -Force

$empty = Get-ChildItem $payload -Recurse -File | Where-Object Length -eq 0
if ($empty) {
    throw "zero-byte files staged, which forge cannot compress:`n  " +
          (($empty | ForEach-Object FullName) -join "`n  ")
}

$files = Get-ChildItem $payload -Recurse -File
$total = ($files | Measure-Object Length -Sum).Sum
Write-Host ("staged {0} files, {1:N1} MB, into {2}" -f $files.Count, ($total / 1MB), $payload)
foreach ($f in $files | Sort-Object Length -Descending | Select-Object -First 4) {
    Write-Host ("  {0,8:N1} MB  {1}" -f ($f.Length / 1MB), $f.Name)
}
