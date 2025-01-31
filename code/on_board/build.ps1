[CmdletBinding()]
Param(
    [switch] $NoBuild,
    [switch] $DebugBuild,
    [switch] $ProgramOOCD,
    [switch] $DebugOOCD,
    [string] $BinaryName = "trainbuttons"
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
    ".\target\thumbv6m-none-eabi\debug\$BinaryName"
} Else {
    ".\target\thumbv6m-none-eabi\release\$BinaryName"
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

If ($DebugOOCD)
{
    $null = Read-Host -Prompt "Hit Enter after resetting."

    $oocd = Start-Process `
        -FilePath "pwsh.exe" `
        -ArgumentList @("-NoExit -Command & \`"C:\Program Files\OpenOCD\bin\openocd.exe\`" -c \`"source oocd-debug-jlink.cfg\`"") `
        -PassThru

    $gdb = Start-Process `
        -FilePath 'C:\Program Files\arm-gcc\bin\arm-none-eabi-gdb.exe' `
        -ArgumentList @("`"-ex`" `"target extended-remote :3333`" `"$elfBinary`"") `
        -PassThru

    Write-Output "Exit GDB and OpenOCD to return to console."
    $oocd.WaitForExit()
    $gdb.WaitForExit()
}
