<#
.SYNOPSIS
Builds, signs and packages the shipping installer.

.DESCRIPTION
The whole path from a clean tree to a signed Setup.exe, in the order the steps
have to happen. Two orderings here are load-bearing and are the reason this is a
script rather than a paragraph in the README:

  The payload binary is signed before it is staged. Payload members are
  extracted verbatim, so an executable that goes into the container unsigned
  comes out of it unsigned, and signing the installer afterwards does nothing
  for it.

  The installer is signed last, by lwforge itself. Signing is what makes the
  bytes immutable, so everything that rewrites the image has to be finished.

Signing needs AZURE_TENANT_ID, AZURE_CLIENT_ID and AZURE_CLIENT_SECRET in the
environment. Nothing else: the credential chain is pinned to EnvironmentCredential
by signing\metadata.json.

.PARAMETER SkipContent
Reuse the existing build\content.db. Embedding 1,227 chunks takes a minute and a
half and nothing about it changes unless content\ did.

.PARAMETER Dev
Skip both signing steps and mark the container as a development build. The stub
says so, so it cannot be mistaken for something shippable.
#>
[CmdletBinding()]
param(
    [switch] $SkipContent,
    [switch] $Dev
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$repo    = Split-Path $PSScriptRoot -Parent
$tools   = Join-Path $repo 'installer\tools'
$sign    = Join-Path $tools 'scripts\sign.ps1'
$exe     = Join-Path $repo 'src-tauri\target\release\coding-compendium.exe'

# The version the installer is named for comes from the config, so the file name
# and the ARP entry cannot disagree about which build this is.
$version = (Select-String -Path (Join-Path $repo 'installer\installer.toml') `
                          -Pattern '^version\s*=\s*"(.+)"').Matches[0].Groups[1].Value
$out = Join-Path $repo "installer\dist\CodingCompendium-Setup-$version.exe"

if (-not $Dev) {
    foreach ($v in 'AZURE_TENANT_ID', 'AZURE_CLIENT_ID', 'AZURE_CLIENT_SECRET') {
        if (-not (Test-Path "env:$v")) {
            throw "$v is not set. Signing needs all three, and only those three. " +
                  "Pass -Dev to build an unsigned development installer instead."
        }
    }
}

function Step([string] $Message) { Write-Host "`n== $Message" -ForegroundColor Cyan }

# The packaging tools are fetched, not vendored, and the version is pinned here.
#
# This is not tidiness. An installer was once forged with tools three releases
# stale, against a payload the newer stub handles and the older one rejects, and
# the error it produced named compression rather than the file responsible.
# Pinning means the tools move when someone decides they should.
$forgeVersion = 'v0.3.0'
$stamp = Join-Path $tools '.version'

Step "forge tools ($forgeVersion)"
if (-not (Test-Path $stamp) -or (Get-Content $stamp -Raw).Trim() -ne $forgeVersion) {
    New-Item -ItemType Directory $tools -Force | Out-Null
    & gh release download $forgeVersion --repo Locke-Werks/Forge `
        --pattern 'lwforge.exe' --pattern 'lwstub.exe' --dir $tools --clobber
    if ($LASTEXITCODE) { throw "could not download Forge $forgeVersion" }
    Set-Content $stamp $forgeVersion
    Write-Host "  fetched"
} else {
    Write-Host "  already at $forgeVersion"
}

# A stub that is not signed cannot produce a signed installer, and lwforge would
# demand --dev. Better to say so here than to discover it after the long build.
$stubSig = Get-AuthenticodeSignature (Join-Path $tools 'lwstub.exe')
if (-not $Dev -and $stubSig.Status -ne 'Valid') {
    throw "lwstub.exe is $($stubSig.Status). A shipping installer needs a signed stub."
}

Step 'frontend'
Push-Location $repo
try { & pnpm build; if ($LASTEXITCODE) { throw 'pnpm build failed' } }
finally { Pop-Location }

if (-not $SkipContent) {
    Step 'corpus'
    Push-Location $repo
    try { & pnpm build:content; if ($LASTEXITCODE) { throw 'pnpm build:content failed' } }
    finally { Pop-Location }
}

Step 'release binary'
# --features custom-protocol is not optional. Without it Tauri builds a binary
# that loads the frontend from the dev server, which fails on any machine that
# is not running `pnpm dev`. See the comment in src-tauri/Cargo.toml.
& cargo build --release --manifest-path (Join-Path $repo 'src-tauri\Cargo.toml') `
    --bin coding-compendium --features custom-protocol
if ($LASTEXITCODE) { throw 'cargo build failed' }

if (-not $Dev) {
    Step 'signing the payload binary'
    & $sign $exe
    if ($LASTEXITCODE) { throw 'signing the payload binary failed' }
}

Step 'staging the payload'
# Splatted from a hashtable rather than a conditional array: an empty array is
# still one positional argument, and stage-payload.ps1 takes none.
$stageArgs = @{}
if ($Dev) { $stageArgs['AllowUnsigned'] = $true }
& (Join-Path $PSScriptRoot 'stage-payload.ps1') @stageArgs

Step 'forging'
New-Item -ItemType Directory (Split-Path $out -Parent) -Force | Out-Null

# Run from installer\tools, with relative paths, because lwforge resolves two
# things against two different bases and only this directory satisfies both.
#
# product.icon resolves against the config file's directory, and the resolved
# path is opened through the \\?\ long-path prefix, which does not collapse
# `..`. An absolute --config on a path like "C:\...\Coding Compendium" therefore
# yields \\?\C:\...\installer\..\src-tauri\... and fails with ERROR_INVALID_NAME.
# Relative paths stay short enough that the prefix is never applied.
#
# --sign shells out to scripts\sign.ps1 relative to the working directory, and
# that script reads signing\metadata.json from its own parent. Both live under
# installer\tools.
Push-Location $tools
try {
    $forgeArgs = @(
        'build'
        '--config',  '..\installer.toml'
        '--payload', '..\payload'
        '--stub',    '.\lwstub.exe'
        '--out',     "..\dist\$(Split-Path $out -Leaf)"
    )
    if ($Dev) { $forgeArgs += '--dev' } else { $forgeArgs += '--sign' }

    & .\lwforge.exe @forgeArgs
    if ($LASTEXITCODE) { throw 'lwforge failed' }
}
finally { Pop-Location }

Step 'result'
$sig = Get-AuthenticodeSignature $out
"{0}`n  {1:N1} MB`n  signature: {2}" -f $out, ((Get-Item $out).Length / 1MB), $sig.Status
if ($sig.Status -eq 'Valid') { "  signer:    $($sig.SignerCertificate.Subject.Split(',')[0])" }
