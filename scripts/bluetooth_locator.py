import nxt.locator # type: ignore

with nxt.locator.find() as b:
    print(b.get_device_info()[0:2])

