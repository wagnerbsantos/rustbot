import pyautogui
import numpy as np
import os
import threading
import time
import keyboard
import random
import traceback
from PIL import Image
from consts import *

ranges = []
# food = 0
random.seed()
pyautogui.PAUSE = 0.01


class Status:
    is_attacking = False
    life_value = 3
    mana_value = 3
    has_enemy = False
    is_following = False
    move_time = -2
    food_time = -2
    stopped_attacking = False
    looted = False




def cropper(image):
    average = [[0] * (FIELD_HEIGHT) for i in range(FIELD_WIDTH)]
    for w in range(FIELD_WIDTH):
        for h in range(FIELD_HEIGHT):
            cropped = image[
                TOP_LEFT_FIELD[1]
                + h * SQUARE_SIZE : TOP_LEFT_FIELD[1]
                + (h + 1) * SQUARE_SIZE,
                TOP_LEFT_FIELD[0]
                + w * SQUARE_SIZE : TOP_LEFT_FIELD[0]
                + (w + 1) * SQUARE_SIZE,
            ]
            average[w][h] = averager(cropped)
    ranger(average)
    print(ranges)


def averager(cropped):
    r = 0
    g = 0
    b = 0
    q = 1
    for line in cropped:
        for pixel in line:
            r = r + pixel[0]
            g = g + pixel[1]
            b = b + pixel[2]
            q = q + 1
    return singleRange((r // q, g // q, b // q), cropped)


def ranger(averages):
    count = 0
    for line in averages:
        for average in line:
            inrange = False
            for r in ranges:
                if isInRange(average, r):
                    inrange = True
                    break
            if not inrange:
                ranges.append(average)
    return ranges


def singleRange(rgb, image):
    inrange = False
    ran = (0, 0, 0)
    for r in ranges:
        if isInRange(rgb, r):
            inrange = True
            ran = r
            break
    if not inrange:
        ranges.append(rgb)
        ran = rgb
    return ran


def isInRange(average, ran):
    offset = 10
    for i in range(3):
        if not (average[i] > ran[i] - offset and average[i] < ran[i] + offset):
            return False

    return True


def getIsAttacking(image):
    if image[IS_ATTACKING_NEAREST_POS1][RED] > IS_ATTACKING_RED:
        return True
    if image[IS_ATTACKING_NEAREST_POS2][RED] > IS_ATTACKING_RED:
        return True
    if image[IS_ATTACKING_NEAREST_POS3][RED] > IS_ATTACKING_RED:
        return True
    if image[IS_ATTACKING_NEAREST_POS4][RED] > IS_ATTACKING_RED:
        return True
    return False


def getLifeValue(image):
    if image[HIGH_LIFE_POS][GREEN] == HAS_LIFE_GREEN:
        return 3
    elif image[MID_LIFE_POS][GREEN] == HAS_LIFE_GREEN:
        return 2
    elif image[LOW_LIFE_POS][GREEN] == HAS_LIFE_GREEN:
        return 1
    return 0


def getManaValue(image):
    if image[HIGH_MANA_POS][BLUE] == HAS_MANA_BLUE:
        return 3
    elif image[MID_MANA_POS][BLUE] == HAS_MANA_BLUE:
        return 2
    elif image[LOW_MANA_POS][BLUE] == HAS_MANA_BLUE:
        return 1
    return 0


def getHasEnemy(image):
    print(tuple(image[FIRST_ENEMY_POS][0:3]))
    if tuple(image[FIRST_ENEMY_POS][0:3]) == ENEMY_EXIST_RGB:
        return True
    return False


def getIsFollowing(image):
    if image[FOLLOW_BUTTON][GREEN] > 250:
        return True
    return False


def getStatus(status: Status, image):
    is_attacking = getIsAttacking(image)
    if status.is_attacking and not is_attacking:
        status.stopped_attacking = True
    status.is_attacking = is_attacking
    status.life_value = getLifeValue(image)
    status.mana_value = getManaValue(image)
    status.has_enemy = getHasEnemy(image)
    status.is_following = getIsFollowing(image)
    status.move_time += 1
    status.food_time += 1


def loot():
    pyautogui.keyDown("shift")
    for square in LOOT_SQUARES:
        x = square[0] 
        y = square[1]
        pyautogui.rightClick(x, y)
        time.sleep(0.0 1)
    pyautogui.keyUp("shift")
    pyautogui.moveTo(tuple(reversed(STANDBY_MOUSE)))


def decision(status: Status, walkables: list):
    if status.stopped_attacking:
        status.looted = True
        loot()
        status.stopped_attacking = False
    if status.life_value < 3 or status.mana_value == 3:
       pyautogui.press(HEAL_HOTKEY)
    if status.food_time % 50 == 0:
        pyautogui.press(FOOD_HOTKEY)
    if status.has_enemy and not status.is_attacking:
        print("attack")
        pyautogui.click(tuple(reversed(FIRST_ENEMY_POS)))
        pyautogui.moveTo(tuple(reversed(STANDBY_MOUSE)))
    if (
        status.move_time % MOVE_DEFAULT_COOLDOWN == 0
        and not status.is_attacking
        and not status.has_enemy
    ):
        move(walkables)
    if status.is_attacking and not status.is_following:
        pyautogui.click(tuple(reversed(WINDOW_OFFSET + FOLLOW_BUTTON)))
        pyautogui.moveTo(tuple(reversed(WINDOW_OFFSET + STANDBY_MOUSE)))


def move(walkables):
    if len(walkables) < 2:
        return
    numero = random.randint(0, len(walkables) - 1)
    pyautogui.click(tuple(reversed(walkables[numero])))
    pyautogui.moveTo(tuple(reversed(STANDBY_MOUSE)), duration=0.1)


def findWalkable(image):
    listao = []
    basew = MAP_TOP_LEFT[0]
    baseh = MAP_TOP_LEFT[1]
    for h in range(5, MAP_HEIGHT, 5):
        for w in range(5, MAP_WIDTH, 5):
            if h < 45 and h > 25 and w < 45 and w > 25:
                continue
            if tuple(image[baseh + h][basew + w][0:3]) == MAP_WALKABLE_RGB:
                listao.append((baseh + h, basew + w))
    return listao


def clearFiles():
    files = os.listdir()
    for file in files:
        os.remove(file)


def main():
    status = Status()
    while True:
        im = np.array(pyautogui.screenshot())
        getStatus(status, im)
        print(status.move_time)
        walkables = findWalkable(im)
        if status.is_attacking:
            status.move_time = -2
        decision(status, walkables)
        #pyautogui.moveTo(tuple(reversed(STANDBY_MOUSE)))
        # cropper(im)
        time.sleep(3)
        if status.looted:
            status.looted = False


if __name__ == "__main__":
    main()                          
