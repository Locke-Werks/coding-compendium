# Generates the app icons Tauri needs.
#
# Run once, or after changing the design:
#   pwsh -File scripts/make-icons.ps1
#
# The output lands in src-tauri/icons/ and IS committed, because tauri-build
# fails without it and a fresh clone should build without running a script first.
#
# The mark is a bracket-and-bar glyph: a code bracket enclosing a stack of lines
# suggesting pages. It has to survive being drawn at 32 pixels, so it is two
# shapes and no detail.

Add-Type -AssemblyName System.Drawing

$iconsDir = Join-Path $PSScriptRoot '..\src-tauri\icons'
if (-not (Test-Path $iconsDir)) { New-Item -ItemType Directory -Path $iconsDir -Force | Out-Null }

function New-CompendiumBitmap {
    param([int]$Size)

    $bmp = New-Object System.Drawing.Bitmap($Size, $Size)
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::AntiAlias
    $g.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic

    # Rounded slate background.
    $bg = [System.Drawing.Color]::FromArgb(255, 24, 26, 33)
    $radius = [int]($Size * 0.22)
    $path = New-Object System.Drawing.Drawing2D.GraphicsPath
    $d = $radius * 2
    $path.AddArc(0, 0, $d, $d, 180, 90)
    $path.AddArc($Size - $d, 0, $d, $d, 270, 90)
    $path.AddArc($Size - $d, $Size - $d, $d, $d, 0, 90)
    $path.AddArc(0, $Size - $d, $d, $d, 90, 90)
    $path.CloseFigure()
    $g.FillPath((New-Object System.Drawing.SolidBrush($bg)), $path)

    # Amber page lines, decreasing width, suggesting a text block.
    $amber = [System.Drawing.Color]::FromArgb(255, 232, 168, 76)
    $lineBrush = New-Object System.Drawing.SolidBrush($amber)
    $lineH = [Math]::Max(1, [int]($Size * 0.075))
    $gap = [int]($Size * 0.135)
    $left = [int]($Size * 0.38)
    $top = [int]($Size * 0.28)
    $widths = @(0.36, 0.28, 0.20)
    for ($i = 0; $i -lt 3; $i++) {
        $w = [int]($Size * $widths[$i])
        $g.FillRectangle($lineBrush, $left, $top + ($i * $gap), $w, $lineH)
    }

    # Left bracket, drawn as a stroke so it reads as code rather than decoration.
    $penW = [Math]::Max(1.5, $Size * 0.075)
    $pen = New-Object System.Drawing.Pen($amber, $penW)
    $pen.StartCap = [System.Drawing.Drawing2D.LineCap]::Round
    $pen.EndCap = [System.Drawing.Drawing2D.LineCap]::Round
    $pen.LineJoin = [System.Drawing.Drawing2D.LineJoin]::Round

    $bx = [int]($Size * 0.30)
    $byTop = [int]($Size * 0.24)
    $byBot = [int]($Size * 0.76)
    $bIn = [int]($Size * 0.10)
    # The array must be typed. An untyped @() is Object[], and PowerShell then
    # picks the single-Point overload of DrawLines and fails to convert.
    [System.Drawing.Point[]]$pts = @(
        (New-Object System.Drawing.Point(($bx + $bIn), $byTop)),
        (New-Object System.Drawing.Point($bx, $byTop)),
        (New-Object System.Drawing.Point($bx, $byBot)),
        (New-Object System.Drawing.Point(($bx + $bIn), $byBot))
    )
    $g.DrawLines($pen, $pts)

    $g.Dispose()
    return $bmp
}

# PNGs, at the sizes tauri.conf.json lists.
foreach ($spec in @(@(32, '32x32.png'), @(128, '128x128.png'), @(256, '128x128@2x.png'), @(512, 'icon.png'))) {
    $bmp = New-CompendiumBitmap -Size $spec[0]
    $out = Join-Path $iconsDir $spec[1]
    $bmp.Save($out, [System.Drawing.Imaging.ImageFormat]::Png)
    $bmp.Dispose()
    Write-Host "wrote $out"
}

# The .ico, hand-assembled.
#
# System.Drawing's Icon.FromHandle path silently downsamples to 32x32 and loses
# the alpha channel, which on Windows 11 produces a muddy taskbar icon. Writing
# the container by hand and embedding real PNGs avoids both problems.
$sizes = @(16, 32, 48, 64, 128, 256)
$images = @()
foreach ($s in $sizes) {
    $bmp = New-CompendiumBitmap -Size $s
    $ms = New-Object System.IO.MemoryStream
    $bmp.Save($ms, [System.Drawing.Imaging.ImageFormat]::Png)
    $images += , @{ Size = $s; Bytes = $ms.ToArray() }
    $ms.Dispose()
    $bmp.Dispose()
}

$icoPath = Join-Path $iconsDir 'icon.ico'
$fs = [System.IO.File]::Create($icoPath)
$bw = New-Object System.IO.BinaryWriter($fs)

# ICONDIR: reserved, type 1 (icon), image count.
$bw.Write([UInt16]0)
$bw.Write([UInt16]1)
$bw.Write([UInt16]$images.Count)

# Each ICONDIRENTRY is 16 bytes and follows the 6-byte header.
$offset = 6 + (16 * $images.Count)
foreach ($img in $images) {
    # 256 is stored as 0, since the field is a single byte.
    $dim = if ($img.Size -ge 256) { 0 } else { $img.Size }
    $bw.Write([Byte]$dim)          # width
    $bw.Write([Byte]$dim)          # height
    $bw.Write([Byte]0)             # palette size, 0 for truecolor
    $bw.Write([Byte]0)             # reserved
    $bw.Write([UInt16]1)           # color planes
    $bw.Write([UInt16]32)          # bits per pixel
    $bw.Write([UInt32]$img.Bytes.Length)
    $bw.Write([UInt32]$offset)
    $offset += $img.Bytes.Length
}
foreach ($img in $images) { $bw.Write($img.Bytes) }

$bw.Flush(); $bw.Dispose(); $fs.Dispose()
Write-Host "wrote $icoPath ($($images.Count) sizes)"
