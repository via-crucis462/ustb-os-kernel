import os

base_address = 0x80400000
step = 0x20000

apps = os.listdir("src/bin")
apps.sort()

app_id = 0
for app in apps:
    if not app.endswith('.rs'):
        continue
    app_name = app[:-3]
    print("[build.py] building {} with address {}".format(app_name, hex(base_address + step * app_id)))
    os.system(
        "cargo rustc --bin %s --release -- -Clink-args=-Ttext=%x"
        % (app_name, base_address + step * app_id)
    )
    print("[build.py] generating binary for {}".format(app_name))
    os.system(
        "rust-objcopy --binary-architecture=riscv64 --strip-all -O binary target/riscv64gc-unknown-none-elf/release/%s target/riscv64gc-unknown-none-elf/release/%s.bin"
        % (app_name, app_name)
    )
    app_id = app_id + 1
