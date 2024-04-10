#!/usr/bin/bash
# every subdirectory of / except /proc, /dev, /sys, /run, /tmp
# as those are virtual file systems and do not contain real files
sudo find /bin /boot /etc /home /lib /lib64 /lost+found /mnt /opt /root /sbin /srv /usr /var \
    -type f -printf '%s\n' | sort -n | uniq -c > result.txt
