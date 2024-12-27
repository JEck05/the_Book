
# All valid cargo installs have a Cargo.toml file so to determine
# if we can clean the directory we can check that it has this file
$cargoPath = "/target"
# Get a list of directories in the current location and run a command on each
Get-ChildItem -Directory | ForEach-Object {
    # $_ represents the current directory

    $validCargoDir = Join-Path -Path $_ -ChildPath $cargoPath
    if(Test-Path -Path $validCargoDir){
        Write-Host "Processing directory: $($_.FullName)"
        Set-Location $_
        cargo clean
        Set-Location ..
    }
}
