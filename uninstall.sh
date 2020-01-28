#!/bin/bash
simpi_uninstall () {
    #[ "$UID" -eq 0 ] || exec sudo bash "$0" "$@"
    sudo cd . 2> /dev/null
    local app_id="simpi"
    local app_name="SimPi Broker"
    local app_data_dir="$HOME/$app_id"
    local prog_files_dir="/opt/$app_id"
    local start_menu_dir="$HOME/.local/share/applications"
    # #0 - Delete folders
    echo Delete Program Files Simpi folder
    sudo rm -rf "$prog_files_dir"
    sudo rm "/usr/include/wiringPi.h"
    sudo rm "/usr/lib/x86_64-linux-gnu/libwpisim.d"
    sudo rm "/usr/lib/x86_64-linux-gnu/libwpisim.so"
    sudo rm "$start_menu_dir/$app_id.desktop"
    # #1 - Done
    read -n 1 -s -r -p "Done! Press any key to continue..."
    echo -e "\n"
}

simpi_uninstall
