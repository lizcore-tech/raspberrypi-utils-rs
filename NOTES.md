```bash
git submodule add https://github.com/raspberrypi/utils raspberrypi-utils-git
```

```bash
# Install dependencies to build raspberrypi-utils
sudo apt install -y cmake device-tree-compiler libfdt-dev libgnutls28-dev
```

Pin repo to a specific commit or tag
```bash
cd raspberrypi-utils-git
git checkout <commit-hash-or-tag>
cd ../..
git add raspberrypi-utils-git
git commit -m "Add raspberrypi/utils submodule pinned to <commit-hash>"
```

When someone else clones the repo (or for your CI), they will need to initialize the submodule:

```bash
git clone --recursive https://github.com/lizcore-tech/raspberrypi-utils-rs.git
# OR if already cloned:
git submodule update --init --recursive
```