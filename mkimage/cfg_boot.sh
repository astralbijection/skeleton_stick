#!/bin/sh

boot_block=$1
ssid=$2
ssid_pw=$3

if [ "$#" -ne 3 ]; then
    echo "Usage: $0 [boot block device] [ssid] [ssid password]"
    exit 1
fi


SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

dir=$(mktemp -d)
mount $boot_block $dir

#cp $SCRIPT_DIR/cmdline.txt $dir/cmdline.txt
#cp $SCRIPT_DIR/config.txt $dir
touch $dir/ssh
echo 'ctrl_interface=DIR=/var/run/wpa_supplicant GROUP=netdev' > $dir/wpa_supplicant.conf
echo 'update_config=1' >> $dir/wpa_supplicant.conf
echo 'country=US' >> $dir/wpa_supplicant.conf
wpa_passphrase "$ssid" "$ssid_pw" >> $dir/wpa_supplicant.conf


umount $dir
