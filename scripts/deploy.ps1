param([String]$Server)
& ./scripts/build.ps1

Write-Host "You input server '$Server'" 

echo "Deploying..."
echo "Send file to caravel hardware"
echo "You need to enter the password"

if ($Server -eq $null) {
    scp -P 1337 .\target\armv7-unknown-linux-gnueabihf\release\caravel pi@10.170.208.45:/home/pi/caravel/
}
else {
    scp .\target\armv7-unknown-linux-gnueabihf\release\caravel pi@${Server}:/home/pi/caravel/
}
