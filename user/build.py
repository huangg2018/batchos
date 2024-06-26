import os

base_address = 0x80400000
step = 0x20000
linker = "src/linker.ld"

app_id = 0
apps = os.listdir("build/app")
apps.sort()
chapter = os.getenv("CHAPTER")
mode = os.getenv("MODE", default = "release")
if mode == "release" :
	mode_arg = "--release"
else :
    mode_arg = ""
print("apps: %s" % apps)
for app in apps:
    app = app[: app.find(".")]
    print("cargo rustc --bin %s %s -- -Clink-args=-Ttext=%x"
        % (app, mode_arg, base_address + step * app_id))
    #cargo rustc --bin ch2b_power_3 --release -- -Clink-args=-Ttext=80400000
    os.system(
        "cargo rustc --bin %s %s -- -Clink-args=-Ttext=%x"
        % (app, mode_arg, base_address + step * app_id)
    )
    print(
        "[build.py] application %s start with address %s"
        % (app, hex(base_address + step * app_id))
    )
    if chapter == '3':
        app_id = app_id + 1
