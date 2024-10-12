# -------------------------------------------------------- #
#  LEGO NXT 2.0 Brain                                      #
#  ------------------------------------------------------- #
# https://ni.srht.site/nxt-python/latest/                  #
# https://github.com/schodet/nxt-python                    #
# -------------------------------------------------------- #
# To check correct permorm, plug via USB and run:
#   >nxt_test
# In case of problem, enable debugging for extra diagnostics:
#   >nxt_test --log-level=debug

#!/usr/bin/python3
"""NXT-Python tutorial: find the brick."""
import nxt.locator
import nxt.motor

# Find a brick.
with nxt.locator.find() as b:
    # Once found, print its name.
    print("Found brick:", b.get_device_info()[0])
    # And play a recognizable note.
    b.play_tone(440, 250)
    # Get the motor connected to the port A.
    mymotor = b.get_motor(nxt.motor.Port.A)
    # Full circle in one direction.
    mymotor.turn(25, 360)
    # Full circle in the opposite direction.
    mymotor.turn(-25, 360)


