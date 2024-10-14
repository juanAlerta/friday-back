import nxt.locator # type: ignore

with nxt.locator.find(host="00:16:53:14:87:D3") as robot:
    robot.play_tone(440, 250)
    # Get the motor connected to the port A.
    mymotor = robot.get_motor(nxt.motor.Port.A)
    # Full circle in one direction.
    mymotor.turn(25, 360)
    # Full circle in the opposite direction.
    mymotor.turn(-25, 360)