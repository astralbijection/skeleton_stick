#!/bin/sh

# A helper script to run this program on a remote device such as a raspberry pi.
# Prior to running this script, you should:
#  - copy over SSH keys
#  - sudo pip install -r requirements.txt

ssh_target=$1
if [ "$#" -lt 1 ]; then
    echo "Usage: $0 ssh_target program_args ..."
    exit 1
fi

shift 1

dir=$(mktemp -d)

script="
mkdir -p $dir
echo \"Copying files to $dir\"
cd $dir

tar x  # extract tar from stdin
"

git ls-files skeleton_stick requirements.txt resources setup.py | tar c --files-from=- | ssh $ssh_target "$script"

ssh -t $ssh_target "cd $dir && sudo python3 -m skeleton_stick $@"