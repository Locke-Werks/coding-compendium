# Screenshot a window by process name, optionally typing something first.
#
#   pwsh -File scripts/shoot.ps1 -Process coding-compendium -Out shot.png
#   pwsh -File scripts/shoot.ps1 -Process coding-compendium -Type "merge conflict" -Out shot.png
#
# Used to check the app actually renders, which no unit test can tell you.
# PrintWindow is used rather than a screen grab so the capture works even when
# the window is behind something else.

param(
    [Parameter(Mandatory = $true)][string]$Process,
    [string]$Type = "",
    [Parameter(Mandatory = $true)][string]$Out,
    [int]$SettleMs = 1200
)

Add-Type -AssemblyName System.Drawing
Add-Type -AssemblyName System.Windows.Forms

Add-Type @'
using System;
using System.Runtime.InteropServices;
public class Win {
    [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr h);
    [DllImport("user32.dll")] public static extern bool ShowWindow(IntPtr h, int cmd);
    [DllImport("user32.dll")] public static extern bool PrintWindow(IntPtr h, IntPtr hdc, uint flags);
    [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
    [DllImport("user32.dll")] public static extern bool SetProcessDPIAware();
    [StructLayout(LayoutKind.Sequential)]
    public struct RECT { public int Left, Top, Right, Bottom; }
}
'@

# Become DPI aware before measuring anything.
#
# Without this, Windows lies to the script: on a 4K panel at 175% scaling
# GetWindowRect returns logical pixels (1214 wide) while the DPI-aware Tauri
# window actually renders at physical resolution (~2100 wide). The capture
# bitmap comes out sized for the lie, and PrintWindow fills it from the top-left
# of the real content, so the right and bottom of the window are silently cut
# off. That looked exactly like a CSS layout bug and cost a round of chasing one.
[Win]::SetProcessDPIAware() | Out-Null

$proc = Get-Process -Name $Process -ErrorAction SilentlyContinue |
    Where-Object { $_.MainWindowHandle -ne 0 } | Select-Object -First 1
if (-not $proc) { Write-Error "no window found for process '$Process'"; exit 1 }

$h = $proc.MainWindowHandle
[Win]::ShowWindow($h, 9) | Out-Null   # SW_RESTORE, in case it is minimized
[Win]::SetForegroundWindow($h) | Out-Null
Start-Sleep -Milliseconds 400

if ($Type) {
    # The search box takes focus on mount, so typing goes straight to it.
    [System.Windows.Forms.SendKeys]::SendWait($Type)
    Start-Sleep -Milliseconds $SettleMs
}

$r = New-Object Win+RECT
[Win]::GetWindowRect($h, [ref]$r) | Out-Null
$w = $r.Right - $r.Left
$hgt = $r.Bottom - $r.Top

$bmp = New-Object System.Drawing.Bitmap($w, $hgt)
$g = [System.Drawing.Graphics]::FromImage($bmp)
$hdc = $g.GetHdc()
# flag 2 = PW_RENDERFULLCONTENT, required for WebView2 content to appear.
[Win]::PrintWindow($h, $hdc, 2) | Out-Null
$g.ReleaseHdc($hdc)
$g.Dispose()

$bmp.Save($Out, [System.Drawing.Imaging.ImageFormat]::Png)
$bmp.Dispose()
Write-Host "wrote $Out ($w x $hgt)"
