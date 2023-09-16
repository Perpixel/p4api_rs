$P4API_VERSION = "r23.1"
$SSL_VERSION = "3.0.10"
$NASM_VERSION = "2.16.01"

if (-not [Environment]::GetEnvironmentVariable('VS2022INSTALLDIR')) { Write-Host "Visual Studio 2022 not installed." }
if (Test-Path -Path "./extern") { Remove-Item ./extern -recurse > $null }
if (Test-Path -Path "./tmp") { Remove-Item ./tmp -recurse > $null }

New-Item -Path .\tmp -ItemType directory
New-Item -Path .\extern -ItemType directory
New-Item -Path .\extern\bin -ItemType directory

# update system path
$env:PATH += ";$pwd\extern\bin;$pwd\extern\perl\perl\bin;${env:VS2022INSTALLDIR}\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64\"

# install 7z command line utility
Invoke-WebRequest -uri https://7-zip.org/a/7zr.exe -outfile .\extern\bin\7zr.exe
Invoke-WebRequest -uri https://7-zip.org/a/7z2301-extra.7z -outfile .\tmp\7zip.7z
cmd.exe /c '7zr.exe x .\tmp\7zip.7z -o.\extern\bin\'

# p4api dependencies
#Invoke-WebRequest -uri https://cdist2.perforce.com/perforce/${P4API_VERSION}/bin.ntx64/p4api_vs2022_static_openssl3.zip -outfile .\tmp\p4api.zip
Invoke-WebRequest -uri https://www.openssl.org/source/openssl-${SSL_VERSION}.tar.gz -outfile .\tmp\openssl.tar.gz

# build tools for openssl
Invoke-WebRequest -uri https://www.nasm.us/pub/nasm/releasebuilds/${NASM_VERSION}/win64/nasm-${NASM_VERSION}-win64.zip -outfile .\tmp\nasm.zip
Invoke-WebRequest -uri https://strawberryperl.com/download/${PERL_VERSION}/strawberry-perl-${PERL_VERSION}-64bit.zip -outfile .\tmp\perl.zip

cmd.exe /c '7za.exe e .\tmp\nasm.zip -aos -o.\extern\bin\'
cmd.exe /c '7za.exe x .\tmp\perl.zip -aos -o.\extern\perl\'
cmd.exe /c '7za.exe x .\tmp\openssl.tar.gz -so | 7za.exe x -si -ttar -aos -o.\extern\'
cmd.exe /c '7za.exe x .\tmp\p4api.zip -o.\extern\'

# build openssl
cd .\extern\openssl-${SSL_VERSION}
cmd.exe /c 'perl Configure'
cmd.exe /c 'nmake'
