from RPi import GPIO
from time import sleep


class Motor(object):
    """docstring for Motor."""
    def __init__(self, pinA, pinB, pinE):
        self.A = pinA
        self.B = pinB
        self.E = pinE

        GPIO.setup(self.A, GPIO.OUT)
        GPIO.setup(self.B, GPIO.OUT)
        GPIO.setup(self.E, GPIO.OUT)

    def forward(self):
        GPIO.output(self.A, GPIO.HIGH)
        GPIO.output(self.B, GPIO.LOW)
        GPIO.output(self.E, GPIO.HIGH)

    def stop(self):
        GPIO.output(self.E, GPIO.LOW)


GPIO.setmode(GPIO.BOARD)

Motor1A = 16
Motor1B = 18
Motor1E = 22

motor = Motor(Motor1A, Motor1B, Motor1E)

print("Turning motor on")
motor.forward()
sleep(2)

print("Stopping motor")
motor.stop()

GPIO.cleanup()
