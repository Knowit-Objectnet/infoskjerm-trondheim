# Script for updating the binary on the Raspberry PI running the infoscreen.

import requests
import os
import subprocess

# Step 1: Get the latest release JSON
response = requests.get(
    "https://api.github.com/repos/Knowit-Objectnet/infoskjerm-trondheim/releases/latest")
release_json = response.json()

# Print the downloaded version
version = release_json.get("tag_name")
print(f"Downloaded version: {version}")

# Check against running version
with open("VERSION", "w+") as file:
    if version == file.read():
        print("No new version")
        exit(0)

# Step 2: Get the "browser_download_url" from the assets
assets = release_json.get("assets", [])
if len(assets) == 0:
    print("No assets found in the release JSON.")
    exit(1)

download_url = assets[0].get("browser_download_url")
if not download_url:
    print("No browser_download_url found in the assets.")
    exit(1)

# Step 3: Kill the already running process by name
subprocess.run(["pkill", "infoskjerm"])

# Step 4: Download the file
filename = download_url.split("/")[-1]
response = requests.get(download_url)
if response.status_code != 200:
    print("Failed to download the file.")
    exit(1)

with open(filename, "wb") as file:
    file.write(response.content)

# Update running version
with open("VERSION", "w+") as file:
    file.write(version)

# Step 5: Chmod +X the file
os.chmod(filename, 0o755)


# Step 6: Run the run.sh script
subprocess.run(["sh", "run.sh"])
