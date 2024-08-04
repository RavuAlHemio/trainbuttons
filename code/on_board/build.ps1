[CmdletBinding()]
Param(
    [switch] $NoBuild,
    [switch] $DebugBuild,
    [switch] $ProgramOOCD
)

$buildSuccess = $true
If (-not $NoBuild) {
    If ($DebugBuild) {
        & cargo build
    } Else {
        & cargo build --release
    }
    If ($LASTEXITCODE -ne 0) {
        $buildSuccess = $false
    }
}
If (-not $buildSuccess) {
    Return
}

$elfBinary = If ($DebugBuild) {
    ".\target\thumbv6m-none-eabi\debug\trainbuttons"
} Else {
    ".\target\thumbv6m-none-eabi\release\trainbuttons"
}

If (-not $NoBuild) {
    & rust-objcopy --output-target=binary "$elfBinary" ".\tb.bin"
}

$kilobytes = (Get-Item -LiteralPath ".\tb.bin").Length / 1024
Write-Output ("{0:#,##0.###} KiB" -f $kilobytes)

If ($ProgramOOCD) {
    & "C:\Program Files\OpenOCD\bin\openocd.exe" `
        -c "set BINFILE tb.bin" `
        -c "source oocd-prog-jlink.cfg"
}
