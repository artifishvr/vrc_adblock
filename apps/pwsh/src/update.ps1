param([string]$hosts_path = "$env:SystemRoot\System32\drivers\etc\hosts")

$is_admin = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $is_admin) {
    Write-Host 'This script must be run as Administrator. Right-click PowerShell and choose "Run as administrator".' -ForegroundColor Red
    exit 1
}

$url = 'https://raw.githubusercontent.com/artifishvr/vrc_adblock/refs/heads/main/blocked_hosts.txt'
$start_marker = "# start of vrc_adblock section don't touch CmZ9p"
$end_marker   = "# end of vrc_adblock section don't touch CmZ9p"

Write-Host 'Getting latest blocked_hosts.txt file from github...'
try {
    $entries = (Invoke-WebRequest -Uri $url -UseBasicParsing -ErrorAction Stop).Content -split "`r?`n" | Where-Object { $_.Trim() }
} catch {
    Write-Host "Couldn't get new hosts file, github down again?" -ForegroundColor Red
    exit 1
}

$hosts = Get-Content -Path $hosts_path

$missing = $entries | Where-Object {
    $domain = ($_ -split '\s+')[1]
    -not ($hosts -match "^\s*0\.0\.0\.0\s+$([regex]::Escape($domain))(\s|$)")
}

if ($missing) {
    try {
        $start_block = [array]::IndexOf(@($hosts | ForEach-Object { $_.Trim() }), $start_marker)
        $end_block   = [array]::IndexOf(@($hosts | ForEach-Object { $_.Trim() }), $end_marker)

        if ($start_idx -ge 0 -and $end_idx -gt $start_idx) {
            $new_hosts = @($hosts[0..($end_idx - 1)]) + $missing + @($hosts[$end_idx..($hosts.Count - 1)])
            Set-Content -Path $hosts_path -Value $new_hosts -ErrorAction Stop
        } else {
            Add-Content -Path $hosts_path -Value (@('', $start_marker) + $missing + $end_marker) -ErrorAction Stop
        }
        Write-Host 'Successfully updated hosts file'
    } catch {
        Write-Host "Failed to update file" -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host 'All entries already present.'
}