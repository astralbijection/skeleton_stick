## USB Gadget setup

See https://www.isticktoit.net/?p=1383

```
echo "dtoverlay=dwc2" | sudo tee -a /boot/config.txt
echo "dwc2" | sudo tee -a /etc/modules
echo "libcomposite" | sudo tee -a /etc/modules
```

## Required libraries

```
# apt install libopenjp2-7-dev
```