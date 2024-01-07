#!/bin/bash
export DISPLAY=:0
export SLINT_FULLSCREEN=1
export RUST_LOG=info

# Function to check Wi-Fi connection
check_wifi() {
    if ! iwconfig wlan0 | grep -q "ESSID:off/any"; then
        return 0  # Wi-Fi is connected
    else
        return 1  # Wi-Fi is not connected
    fi
}

# Wait for Wi-Fi connection
while ! check_wifi; do
    echo "Waiting for Wi-Fi connection..."
    sleep 5
done

/home/infoskjerm/bin/infoskjerm >> infoskjerm.log 2>&1 &&