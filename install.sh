#!/bin/bash
simpi_install () {
    #[ "$UID" -eq 0 ] || exec sudo bash "$0" "$@"
    sudo cd . 2> /dev/null
    local app_id="simpi"
    local app_name="SimPi Broker"
    local app_data_dir="$HOME/$app_id"
    local prog_files_dir="/opt/$app_id"
    local start_menu_dir="$HOME/.local/share/applications"
    # #0 - Setup folders
    echo "Setting up folders..."
    mkdir -p "$app_data_dir"
    sudo mkdir -p "$prog_files_dir"
    # #1 - Setup SimPi Broker
    echo "Building SimPi Broker..."
    cd broker
    cargo build --release
    echo "Copy files to program install location..."
    sudo cp "./target/release/simpi_broker" "$prog_files_dir"
    sudo cp "./media/app_icon.ico" "$prog_files_dir"
    echo "Create start menu entry..."
    touch "$start_menu_dir/$app_id.desktop"
    echo "[Desktop Entry]
    Name=$app_name
    Exec=$prog_files_dir/simpi_broker
    Path=$prog_files_dir
    Terminal=true
    Type=Application
    Icon=$prog_files_dir/app_icon.ico
    Categories=Development;" >> "$start_menu_dir/$app_id.desktop"
    cd ..
    # #2 - Setup wiringPi lib
    echo "Building wiringPi lib..."
    cd wpisim
    cargo build --release
    sudo cp "./target/release/libwpisim.d" "/usr/lib/x86_64-linux-gnu"
    sudo cp "./target/release/libwpisim.so" "/usr/lib/x86_64-linux-gnu"
    sudo cp "./wiringPi.h" "/usr/include"
    cd ..
    # #3 - Copy LICENSE, README.md and uninstall.sh
    sudo cp "./LICENSE" "$prog_files_dir"
    sudo cp "./README.md" "$prog_files_dir"
    sudo cp "./uninstall.sh" "$prog_files_dir"
    # #4 - Done
    read -n 1 -s -r -p "Done! Press any key to continue..."
    echo -e "\n"
}

simpi_install
