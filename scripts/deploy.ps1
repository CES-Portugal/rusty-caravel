& ./scripts/build.ps1

echo "Deploying..."
echo "Send file to caravel hardware"
echo "You need to enter the password"

scp -P 1337 .\target\armv7-unknown-linux-gnueabihf\release\caravel pi@10.170.208.45:/home/pi/caravel/
