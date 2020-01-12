#!/bin/bash
simpi_uninstall () {
    [ "$UID" -eq 0 ] || exec sudo bash "$0" "$@"
    local app_id="simpi"
    local app_name="SimPi Broker"
    local app_data_dir="$HOME/$app_id"
    local prog_files_dir="/opt/$app_id"
    local start_menu_dir="$HOME/.local/share/applications"
    # #0 - Delete folders
    echo Delete Program Files Simpi folder
    rm -rf "$prog_files_dir"
    rm "/usr/include/wiringPi.h"
    rm "/usr/lib/x86_64-linux-gnu/libwpisim.d"
    rm "/usr/lib/x86_64-linux-gnu/libwpisim.so"
    rm "$start_menu_dir/$app_id.desktop"
    # #1 - Done
    read -n 1 -s -r -p "Done! Press any key to continue..."
    echo -e "\n"
}

simpi_uninstall
