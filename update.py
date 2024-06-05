#!/usr/bin/env python3
# Script for updating the binary on the Raspberry PI running the infoscreen.

import requests
import io
import subprocess
import tarfile

from pathlib import Path

# Step 1: Get the latest release JSON
response = requests.get(
    "https://api.github.com/repos/Knowit-Objectnet/infoskjerm-trondheim/releases/latest")
release_json = response.json()

# Print the downloaded version
version = release_json.get("tag_name")
print(f"Downloaded version: {version}")

# Check against running version
p = Path("VERSION")
p.touch()
with open(p, "r") as file:
    if version == file.read():
        print("No new version")
        exit(0)

# Step 2: Download release
assets = release_json.get("assets", [])
if len(assets) == 0:
    print("No assets found in the release JSON.")
    exit(1)

download_url = assets[0].get("browser_download_url")
if not download_url:
    print("No browser_download_url found in the assets.")
    exit(1)

response = requests.get(download_url)
if response.status_code != 200:
    print("Failed to download the file.")
    exit(1)

# Step 3: Kill the already running process by name
subprocess.run(["pkill", "infoskjerm"])

# Step 4: Unpack update
tarlike = io.BytesIO(response.content)
with tarfile.open(fileobj=tarlike, mode="r:gz") as tar:
    tar.extractall()

# Step 5: Update running version
with open("VERSION", "w+") as file:
    file.write(version)

# Step 6: Run the run.sh script
subprocess.run(["sh", "run.sh"])
