from  pyautogui import press;
import time

def press_keys():
    press('n')
    press('enter')
    print("Keys pressed: 'y', Enter")
time.sleep(10)
while True:
    press_keys()
    time.sleep(.1)  # wait for 2 seconds before pressing the keys again

