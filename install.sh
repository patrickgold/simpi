#!/bin/bash
simpi_install () {
    [ "$UID" -eq 0 ] || exec sudo bash "$0" "$@"
    local app_id="simpi"
    local app_name="SimPi Broker"
    local app_data_dir="$HOME/$app_id"
    local prog_files_dir="/opt/$app_id"
    local start_menu_dir="$HOME/.local/share/applications"
    # #0 - Setup folders
    echo "Setting up folders..."
    mkdir -p "$app_data_dir"
    mkdir -p "$prog_files_dir"
    # #1 - Setup SimPi Broker
    echo "Building SimPi Broker..."
    cd broker
    cargo build --release
    echo "Copy files to program install location..."
    cp "./target/release/simpi_broker" "$prog_files_dir"
    cp "./media/app_icon.ico" "$prog_files_dir"
    echo "Create start menu entry..."
    touch "$start_menu_dir/$app_id.desktop"
    echo "[Desktop Entry]"                              >> "$start_menu_dir/$app_id.desktop"
    echo "Name=$app_name"                               >> "$start_menu_dir/$app_id.desktop"
    echo "Exec=$prog_files_dir/simpi_broker"            >> "$start_menu_dir/$app_id.desktop"
    echo "Path=$prog_files_dir"                         >> "$start_menu_dir/$app_id.desktop"
    echo "Terminal=true"                                >> "$start_menu_dir/$app_id.desktop"
    echo "Type=Application"                             >> "$start_menu_dir/$app_id.desktop"
    echo "Icon=$prog_files_dir/app_icon.ico"  >> "$start_menu_dir/$app_id.desktop"
    echo "Categories=Development;"                      >> "$start_menu_dir/$app_id.desktop"
    cd ..
    # #2 - Setup wiringPi lib
    echo "Building wiringPi lib..."
    cd wpisim
    cargo build --release
    cp "./target/release/libwpisim.d" "/usr/lib/x86_64-linux-gnu/libwipisim.d"
    cp "./target/release/libwpisim.so" "/usr/lib/x86_64-linux-gnu/libwipisim.so"
    cp "./wiringPi.h" "/usr/include/wiringPi.h"
    cd ..
    # #3 - Copy LICENSE, README.md and uninstall.sh
    cp "./LICENSE" "$prog_files_dir"
    cp "./README.md" "$prog_files_dir"
    cp "./uninstall.sh" "$prog_files_dir"
    # #4 - Done
    read -n 1 -s -r -p "Done! Press any key to continue..."
    echo -e "\n"
}

simpi_install
